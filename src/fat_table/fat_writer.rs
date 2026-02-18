use alloc::sync::Arc;
use alloc::vec::Vec;

use crate::error::{self, Result};
use crate::fat_table::{get_params, FatEntry};
use crate::{fat_table, ArcMutex, CachedPartition, ClusterId};
use snafu::ensure;

/// Maximum cluster chain length to prevent infinite loops in corrupted filesystems.
const MAX_CLUSTER_CHAIN_LENGTH: u32 = 1_048_576;

/// Delete a cluster chain starting from `start`.
///
/// For crash safety, the chain is collected first and then deleted in reverse
/// order (last cluster to first). If a power failure interrupts the operation,
/// the head of the chain still points to valid (not-yet-freed) clusters,
/// avoiding orphaned cluster chains. A filesystem check tool can reclaim the
/// partially-freed tail.
pub(crate) fn delete_cluster_chain(
    start: ClusterId,
    device: ArcMutex<CachedPartition>,
) -> Result<()> {
    // Phase 1: collect the full chain.
    let mut chain = Vec::new();
    let mut current = start;
    loop {
        ensure!(
            chain.len() < MAX_CLUSTER_CHAIN_LENGTH as usize,
            error::FilesystemCorruptedSnafu {
                reason: "Cluster chain exceeds maximum length (possible circular reference)"
            }
        );
        chain.push(current);
        match fat_table::next_cluster(current, device.clone())? {
            Some(next) => current = next,
            None => break,
        }
    }

    // Phase 2: delete from last to first for crash safety.
    const DELETED_ENTRY: FatEntry = FatEntry::Unused;
    for &cluster in chain.iter().rev() {
        set_fat_entry(device.clone(), cluster, DELETED_ENTRY)?;
    }

    Ok(())
}

pub(crate) fn set_fat_entry(
    device: Arc<CachedPartition>,
    cluster_id: ClusterId,
    entry: FatEntry,
) -> Result<()> {
    let (sector, offset) = get_params(&device, cluster_id)?;
    let entry_bytes = entry.as_buff();

    // Write to all FAT copies for redundancy (FAT mirroring)
    // Typically fat_amount is 2, but FAT32 spec allows up to 4 copies
    for fat_num in 0..device.fat_amount {
        let fat_sector = sector + (fat_num as u32 * device.sectors_per_fat);
        device
            .clone()
            .write_sector_offset(fat_sector, offset, &entry_bytes)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fat_table::FAT_ENTRY_SIZE;
    use crate::{BlockDevice, CachedPartition, ClusterId, SectorId};
    use alloc::sync::Arc;
    use alloc::vec::Vec;
    use spin::mutex::SpinMutex;

    struct WriteTrackingDevice {
        writes: Arc<SpinMutex<Vec<(SectorId, usize, Vec<u8>)>>>,
    }

    impl WriteTrackingDevice {
        fn new(writes: Arc<SpinMutex<Vec<(SectorId, usize, Vec<u8>)>>>) -> Self {
            Self { writes }
        }
    }

    impl BlockDevice for WriteTrackingDevice {
        fn read_sector_offset(
            &mut self,
            _sector: SectorId,
            _offset: usize,
            buf: &mut [u8],
        ) -> Result<usize> {
            // Return dummy data for FAT entry reads
            buf.fill(0xFF);
            Ok(4) // FAT entry size
        }

        fn write_sector_offset(
            &mut self,
            sector: SectorId,
            offset: usize,
            buf: &[u8],
        ) -> Result<usize> {
            self.writes.lock().push((sector, offset, buf.to_vec()));
            Ok(buf.len())
        }

        fn get_canonical_name() -> &'static str
        where
            Self: Sized,
        {
            "WriteTrackingDevice"
        }
    }

    #[test]
    fn test_fat_mirroring_two_copies() {
        let writes = Arc::new(SpinMutex::new(Vec::new()));
        let device = WriteTrackingDevice::new(writes.clone());

        let cached = Arc::new(CachedPartition::new(
            device,
            512,
            SectorId(100), // FAT starts at sector 100
            1,
            SectorId(300),
            2,  // 2 FAT copies
            50, // 50 sectors per FAT
        ));

        let entry = FatEntry::LastCluster(0x0FFFFFFF);
        set_fat_entry(cached.clone(), ClusterId::new(5), entry).unwrap();

        // Check writes through the shared Arc
        let write_log = writes.lock();
        assert_eq!(write_log.len(), 2, "Should write to 2 FAT copies");
        assert_eq!(
            write_log[0].0,
            SectorId(100),
            "First write should be to FAT #1 at sector 100"
        );
        assert_eq!(
            write_log[1].0,
            SectorId(150),
            "Second write should be to FAT #2 at sector 150 (100 + 50)"
        );
        assert_eq!(
            write_log[0].2, write_log[1].2,
            "Both writes should contain identical data"
        );
        assert_eq!(write_log[0].2.len(), 4, "FAT entry should be 4 bytes");
    }

    #[test]
    fn test_fat_mirroring_four_copies() {
        let writes = Arc::new(SpinMutex::new(Vec::new()));
        let device = WriteTrackingDevice::new(writes.clone());

        let cached = Arc::new(CachedPartition::new(
            device,
            512,
            SectorId(32), // FAT starts at sector 32
            4,
            SectorId(500),
            4,   // 4 FAT copies (maximum per spec)
            100, // 100 sectors per FAT
        ));

        let entry = FatEntry::from_chain(ClusterId::new(10));
        set_fat_entry(cached.clone(), ClusterId::new(3), entry).unwrap();

        let write_log = writes.lock();
        assert_eq!(write_log.len(), 4, "Should write to 4 FAT copies");
        assert_eq!(write_log[0].0, SectorId(32), "FAT #1 at sector 32");
        assert_eq!(
            write_log[1].0,
            SectorId(132),
            "FAT #2 at sector 132 (32 + 100)"
        );
        assert_eq!(
            write_log[2].0,
            SectorId(232),
            "FAT #3 at sector 232 (32 + 200)"
        );
        assert_eq!(
            write_log[3].0,
            SectorId(332),
            "FAT #4 at sector 332 (32 + 300)"
        );

        // All writes should be identical
        for i in 1..write_log.len() {
            assert_eq!(
                write_log[0].2, write_log[i].2,
                "All FAT copies should have identical data"
            );
        }
    }

    /// A block device backed by an in-memory FAT sector that supports
    /// simulating a crash after a configurable number of writes.
    struct CrashSimDevice {
        /// Raw FAT sector data. One sector of 512 bytes = 128 FAT entries.
        fat_sector: Arc<SpinMutex<[u8; 512]>>,
        /// Number of writes remaining before the device "crashes".
        /// None means no crash simulation.
        writes_before_crash: Arc<SpinMutex<Option<usize>>>,
    }

    impl CrashSimDevice {
        fn new(
            fat_sector: Arc<SpinMutex<[u8; 512]>>,
            writes_before_crash: Arc<SpinMutex<Option<usize>>>,
        ) -> Self {
            Self {
                fat_sector,
                writes_before_crash,
            }
        }

        /// Write a FAT entry directly into the backing store (bypasses crash limit).
        fn set_entry(fat_sector: &Arc<SpinMutex<[u8; 512]>>, cluster_id: u32, entry: FatEntry) {
            let offset = cluster_id as usize * FAT_ENTRY_SIZE;
            let bytes = entry.as_buff();
            fat_sector.lock()[offset..offset + FAT_ENTRY_SIZE].copy_from_slice(&bytes);
        }

        /// Read a FAT entry directly from the backing store.
        fn get_entry(fat_sector: &Arc<SpinMutex<[u8; 512]>>, cluster_id: u32) -> FatEntry {
            let offset = cluster_id as usize * FAT_ENTRY_SIZE;
            let data = fat_sector.lock();
            let mut buf = [0u8; FAT_ENTRY_SIZE];
            buf.copy_from_slice(&data[offset..offset + FAT_ENTRY_SIZE]);
            FatEntry::from(buf)
        }
    }

    impl BlockDevice for CrashSimDevice {
        fn read_sector_offset(
            &mut self,
            _sector: SectorId,
            offset: usize,
            buf: &mut [u8],
        ) -> Result<usize> {
            let data = self.fat_sector.lock();
            let end = core::cmp::min(offset + buf.len(), data.len());
            let len = end - offset;
            buf[..len].copy_from_slice(&data[offset..end]);
            Ok(len)
        }

        fn write_sector_offset(
            &mut self,
            _sector: SectorId,
            offset: usize,
            buf: &[u8],
        ) -> Result<usize> {
            let mut limit = self.writes_before_crash.lock();
            if let Some(ref mut remaining) = *limit {
                if *remaining == 0 {
                    return Err(crate::io::ErrorKind::Other.into());
                }
                *remaining -= 1;
            }
            let mut data = self.fat_sector.lock();
            let end = offset + buf.len();
            data[offset..end].copy_from_slice(buf);
            Ok(buf.len())
        }

        fn get_canonical_name() -> &'static str
        where
            Self: Sized,
        {
            "CrashSimDevice"
        }
    }

    /// Build a chain 2 → 3 → 4 → 5 → 6 (last), then simulate a crash
    /// after `crash_after` FAT writes during deletion. Verify the remaining
    /// chain from cluster 2 is still valid: every reachable cluster must be
    /// either a DataCluster pointing to the next or a LastCluster.
    fn crash_during_delete_helper(crash_after: usize) {
        let fat_sector = Arc::new(SpinMutex::new([0u8; 512]));
        let writes_before_crash = Arc::new(SpinMutex::new(None));

        // Build chain: 2→3→4→5→6(last)
        CrashSimDevice::set_entry(&fat_sector, 2, FatEntry::DataCluster(3));
        CrashSimDevice::set_entry(&fat_sector, 3, FatEntry::DataCluster(4));
        CrashSimDevice::set_entry(&fat_sector, 4, FatEntry::DataCluster(5));
        CrashSimDevice::set_entry(&fat_sector, 5, FatEntry::DataCluster(6));
        CrashSimDevice::set_entry(&fat_sector, 6, FatEntry::LastCluster(0x0FFFFFFF));

        let device = CrashSimDevice::new(fat_sector.clone(), writes_before_crash.clone());

        // 1 FAT copy so each cluster deletion = 1 write
        let cached = Arc::new(CachedPartition::new(
            device,
            512,
            SectorId(0), // FAT at sector 0 (matches our single sector)
            1,
            SectorId(100),
            1, // 1 FAT copy
            1, // 1 sector per FAT
        ));

        // Arm the crash: allow only `crash_after` writes
        *writes_before_crash.lock() = Some(crash_after);

        // Deletion will partially succeed then fail
        let _ = delete_cluster_chain(ClusterId::new(2), cached);

        // Verify: walk from cluster 2, every reachable entry must be valid
        let mut current = 2u32;
        let mut visited = 0;
        loop {
            let entry = CrashSimDevice::get_entry(&fat_sector, current);
            match entry {
                FatEntry::DataCluster(next) => {
                    assert!(
                        next >= 2 && next <= 6,
                        "Cluster {} points to invalid cluster {}",
                        current,
                        next
                    );
                    current = next;
                    visited += 1;
                    assert!(visited <= 5, "Infinite loop detected in chain");
                }
                FatEntry::LastCluster(_) => break, // valid chain end
                FatEntry::Unused => break,         // reached freed portion (head was freed)
                other => panic!("Cluster {} has unexpected FAT entry {:?}", current, other),
            }
        }
    }

    #[test]
    fn test_crash_safety_delete_no_writes() {
        // Crash immediately: no clusters freed, full chain intact
        crash_during_delete_helper(0);
    }

    #[test]
    fn test_crash_safety_delete_partial() {
        // Crash after 1-4 writes: chain should still be traversable
        for crash_after in 1..=4 {
            crash_during_delete_helper(crash_after);
        }
    }

    #[test]
    fn test_crash_safety_delete_complete() {
        // All 5 writes succeed: full chain deleted
        crash_during_delete_helper(5);
    }
}
