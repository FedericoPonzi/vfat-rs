use alloc::sync::Arc;

use crate::error::{self, Result};
use crate::fat_table::{get_params, FatEntry};
use crate::{fat_table, ArcMutex, CachedPartition, ClusterId};
use snafu::ensure;

/// Maximum cluster chain length to prevent infinite loops in corrupted filesystems.
const MAX_CLUSTER_CHAIN_LENGTH: u32 = 1_048_576;

/// Delete a cluster chain starting from `current`.
/// TODO: Start from the end of the chain to make the operation safer.
/// TODO: Check if "current" is of "Used" type.
/// TODO: Test with array backed dev.
pub(crate) fn delete_cluster_chain(
    mut current: ClusterId,
    device: ArcMutex<CachedPartition>,
) -> Result<()> {
    const DELETED_ENTRY: FatEntry = FatEntry::Unused;
    let mut iterations = 0;
    while let Some(next) = fat_table::next_cluster(current, device.clone())? {
        ensure!(
            iterations < MAX_CLUSTER_CHAIN_LENGTH,
            error::FilesystemCorruptedSnafu {
                reason: "Cluster chain exceeds maximum length (possible circular reference)"
            }
        );
        set_fat_entry(device.clone(), current, DELETED_ENTRY)?;
        current = next;
        iterations += 1;
    }

    set_fat_entry(device, current, DELETED_ENTRY)?;

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
        let fat_sector = crate::SectorId(sector.0 + (fat_num as u32 * device.sectors_per_fat));
        device
            .clone()
            .write_sector_offset(fat_sector, offset, &entry_bytes)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
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
        ) -> crate::Result<usize> {
            // Return dummy data for FAT entry reads
            buf.fill(0xFF);
            Ok(4) // FAT entry size
        }

        fn write_sector_offset(
            &mut self,
            sector: SectorId,
            offset: usize,
            buf: &[u8],
        ) -> crate::Result<usize> {
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
}
