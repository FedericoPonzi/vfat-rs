use alloc::sync::Arc;
use core::fmt;

use binrw::BinReaderExt;
use binrw::io::Cursor;
use log::{debug, info, trace};
use snafu::ensure;
use spin::mutex::SpinMutex;
use spin::rwlock::RwLock;

use crate::alloc::string::ToString;
use crate::cluster::{cluster_reader, cluster_writer};
use crate::fat_table::FAT_ENTRY_SIZE;
use crate::fat_table::FatEntry;
use crate::formats::extended_bios_parameter_block::{
    BiosParameterBlock, ExtendedBiosParameterBlock, FullExtendedBIOSParameterBlock,
};
use crate::formats::fsinfo::FSInfoSector;
use crate::{
    ArcMutex, Attributes, BlockDevice, CachedPartition, ClusterId, Directory, DirectoryEntry,
    EBPF_VFAT_MAGIC, EBPF_VFAT_MAGIC_ALT, Metadata, RegularDirectoryEntry, SectorId,
    UnknownDirectoryEntry, VfatDirectoryEntry, VfatRsError, fat_table,
};
use crate::{PathBuf, SECTOR_SIZE, TimeManagerTrait};
use crate::{Result, VfatMetadataTrait, error};

/// Maximum cluster chain length to prevent infinite loops in corrupted filesystems.
/// 2^20 = 1,048,576 iterations supports files up to 512GB with 512KB clusters.
const MAX_CLUSTER_CHAIN_LENGTH: u32 = 1_048_576;

/// Main entry point for your VFAT filesystem.
///
/// Every file and directory object will keep a copy of this struct.
///
/// ## Thread Safety
///
/// `VfatFS` uses an internal `RwLock` to synchronize access. Read operations
/// (listing directories, reading files) can proceed concurrently. Write
/// operations (creating/deleting files, writing data) are serialized.
///
/// Individual [`File`](crate::api::File) objects are **not** `Sync` — do not
/// share a single `File` across threads. Instead, open the file independently
/// from each thread.
#[derive(Clone)]
pub struct VfatFS {
    // we need arc around device, because _maybe_ something might need to `Send` this device or Vfat
    // to a different thread.
    pub(crate) device: ArcMutex<CachedPartition>,
    /// Sector of the file allocation table
    pub(crate) fat_start_sector: SectorId,
    /// How many sectors each fat table uses.
    pub(crate) sectors_per_fat: u32,
    /// Id for the root_cluster
    pub(crate) root_cluster: ClusterId,
    /// End of chain marker
    pub(crate) eoc_marker: FatEntry,
    // heap allocated to mostly to ease api
    pub(crate) time_manager: Arc<dyn TimeManagerTrait>,
    /// Filesystem-wide lock: read operations take a shared (read) lock,
    /// write/mutating operations take an exclusive (write) lock.
    pub(crate) fs_lock: Arc<RwLock<()>>,
    /// Hint for the next free cluster search start position (from FSInfo sector).
    /// Updated after each successful allocation to avoid re-scanning used clusters.
    last_alloc_hint: Arc<SpinMutex<u32>>,
    /// Sector number of the FSInfo sector (absolute), or `None` if not present.
    fsinfo_sector: Option<SectorId>,
}

impl fmt::Debug for VfatFS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VfatFilesystem")
    }
}

impl VfatFS {
    /// Create a new VFat filesystem with a default time manager.
    pub fn new<B: BlockDevice + Send + 'static>(
        device: B,
        partition_start_sector: u32,
    ) -> Result<Self> {
        #[cfg(feature = "std")]
        let tm = crate::time::TimeManagerChronos::new();
        #[cfg(not(feature = "std"))]
        let tm = crate::time::TimeManagerNoop::new();
        Self::new_tm(device, partition_start_sector, tm)
    }

    /// Create a new VFat filesystem using a custom time manager.
    pub fn new_tm<B: BlockDevice + Send + 'static>(
        device: B,
        partition_start_sector: u32,
        time_manager: impl TimeManagerTrait + 'static,
    ) -> Result<Self> {
        Self::new_with_cache(device, partition_start_sector, time_manager, 0)
    }

    /// Create a new VFat filesystem with a custom time manager and sector cache.
    ///
    /// `cache_capacity` is the maximum number of sectors to cache in memory.
    /// Use 0 to disable caching (all I/O goes directly to the device).
    pub fn new_with_cache<B: BlockDevice + Send + 'static>(
        mut device: B,
        partition_start_sector: u32,
        time_manager: impl TimeManagerTrait + 'static,
        cache_capacity: usize,
    ) -> Result<Self> {
        let time_manager = Arc::new(time_manager);
        let full_ebpb = Self::read_fullebpb(&mut device, partition_start_sector)?;
        Self::new_with_ebpb(
            device,
            partition_start_sector,
            full_ebpb,
            time_manager,
            cache_capacity,
        )
    }

    /// Read the Full Extended BIOS Parameter block from the device.
    pub fn read_fullebpb<B: BlockDevice + 'static>(
        device: &mut B,
        start_sector: u32,
    ) -> Result<FullExtendedBIOSParameterBlock> {
        let mut buff = [0u8; SECTOR_SIZE];
        device.read_sector(start_sector.into(), &mut buff)?;
        Ok(Cursor::new(&buff).read_le()?)
    }

    /// Validate the BPB fields to prevent panics or undefined behavior
    /// from corrupt or malicious filesystem images.
    fn validate_bpb(bpb: &BiosParameterBlock, ebpb: &ExtendedBiosParameterBlock) -> Result<()> {
        ensure!(
            ebpb.signature == EBPF_VFAT_MAGIC || ebpb.signature == EBPF_VFAT_MAGIC_ALT,
            error::InvalidVfatSnafu {
                target: ebpb.signature
            }
        );
        let bytes_per_sector = bpb.bytes_per_sector;
        ensure!(
            (512..=4096).contains(&bytes_per_sector) && bytes_per_sector.is_power_of_two(),
            error::FilesystemCorruptedSnafu {
                reason: "bytes_per_sector must be 512, 1024, 2048, or 4096"
            }
        );
        ensure!(
            bpb.sectors_per_cluster > 0 && bpb.sectors_per_cluster.is_power_of_two(),
            error::FilesystemCorruptedSnafu {
                reason: "sectors_per_cluster must be a power of 2 and non-zero"
            }
        );
        ensure!(
            bpb.reserved_sectors > 0,
            error::FilesystemCorruptedSnafu {
                reason: "reserved_sectors must be non-zero"
            }
        );
        ensure!(
            bpb.fat_amount > 0,
            error::FilesystemCorruptedSnafu {
                reason: "fat_amount must be non-zero"
            }
        );
        ensure!(
            ebpb.sectors_per_fat > 0,
            error::FilesystemCorruptedSnafu {
                reason: "sectors_per_fat must be non-zero"
            }
        );
        ensure!(
            ebpb.root_cluster >= 2,
            error::FilesystemCorruptedSnafu {
                reason: "root_cluster must be >= 2"
            }
        );
        Ok(())
    }

    /// start_sector: Partition's start sector, or "Entry Offset Sector".
    fn new_with_ebpb<B: BlockDevice + Send + 'static>(
        mut device: B,
        partition_start_sector: u32,
        full_ebpb: FullExtendedBIOSParameterBlock,
        time_manager: Arc<dyn TimeManagerTrait>,
        cache_capacity: usize,
    ) -> Result<Self> {
        Self::validate_bpb(&full_ebpb.bpb, &full_ebpb.extended)?;
        let fat_start_sector =
            (partition_start_sector + full_ebpb.bpb.reserved_sectors as u32).into();
        let fats_total_size = full_ebpb.extended.sectors_per_fat * full_ebpb.bpb.fat_amount as u32;
        let data_start_sector =
            fat_start_sector + fats_total_size + full_ebpb.sectors_occupied_by_all_fats();

        let sectors_per_cluster = full_ebpb.bpb.sectors_per_cluster as u32;
        let root_cluster = ClusterId::new(full_ebpb.extended.root_cluster);
        let eoc_marker = Self::read_end_of_chain_marker(&mut device, fat_start_sector)?;
        let sector_size = device.sector_size();
        let fat_amount = full_ebpb.bpb.fat_amount;
        let sectors_per_fat = full_ebpb.extended.sectors_per_fat;

        // Read the FSInfo sector to get the free-cluster allocation hint.
        let raw_fsinfo_sector = full_ebpb.extended.fsinfo_sector;
        let (alloc_hint, fsinfo_abs_sector) =
            if raw_fsinfo_sector > 0 && raw_fsinfo_sector != 0xFFFF {
                let abs_sector = SectorId::from(partition_start_sector + raw_fsinfo_sector as u32);
                let hint = Self::read_fsinfo_hint(&mut device, abs_sector).unwrap_or(2);
                (hint, Some(abs_sector))
            } else {
                (2, None)
            };

        let cached_partition = CachedPartition::new_with_cache(
            device,
            sector_size,
            fat_start_sector,
            sectors_per_cluster,
            data_start_sector,
            fat_amount,
            sectors_per_fat,
            cache_capacity,
        );
        Ok(VfatFS {
            device: Arc::new(cached_partition),
            fat_start_sector,
            root_cluster,
            eoc_marker,
            sectors_per_fat,
            time_manager,
            fs_lock: Arc::new(RwLock::new(())),
            last_alloc_hint: Arc::new(SpinMutex::new(alloc_hint)),
            fsinfo_sector: fsinfo_abs_sector,
        })
    }

    /// Read the FSInfo sector and return the next-free cluster hint.
    /// Returns `None` on any error or invalid signatures.
    fn read_fsinfo_hint<B: BlockDevice>(device: &mut B, sector: SectorId) -> Option<u32> {
        let mut buf = [0u8; SECTOR_SIZE];
        device.read_sector(sector, &mut buf).ok()?;
        let fsinfo: FSInfoSector = Cursor::new(&buf).read_le().ok()?;
        if !fsinfo.is_valid() {
            return None;
        }
        fsinfo.next_free_hint()
    }

    fn read_end_of_chain_marker<B>(device: &mut B, fat_start_sector: SectorId) -> Result<FatEntry>
    where
        B: BlockDevice,
    {
        const ENTRIES_BUF_SIZE: usize = 1;
        const BUF_SIZE: usize = FAT_ENTRY_SIZE * ENTRIES_BUF_SIZE;
        let mut buf = [0; BUF_SIZE];
        device.read_sector(fat_start_sector, &mut buf)?;
        let raw_entry = FatEntry::from(buf);
        info!("End of chain marker: {:?}", raw_entry);
        Ok(raw_entry)
    }

    fn new_last_cluster_fat_entry(&self) -> FatEntry {
        // Last cluster is initialized with the eoc_marker
        FatEntry::LastCluster(self.eoc_marker.into())
    }

    /// Find next free cluster, starting from the last-allocation hint.
    ///
    /// Scans from the hint to the end of the FAT, then wraps around from
    /// sector 0 to the hint. This avoids re-scanning already-allocated
    /// clusters at the beginning of the FAT.
    pub(crate) fn find_free_cluster(&self) -> Result<Option<ClusterId>> {
        info!("Starting find free cluster routine");
        const ENTRIES_PER_SECTOR: usize = SECTOR_SIZE / FAT_ENTRY_SIZE;
        const BUF_SIZE: usize = FAT_ENTRY_SIZE * ENTRIES_PER_SECTOR;

        let hint = *self.last_alloc_hint.lock();
        let hint_sector = hint / ENTRIES_PER_SECTOR as u32;
        let hint_offset = hint as usize % ENTRIES_PER_SECTOR;

        // Scan from hint_sector..end, then 0..hint_sector (wrap-around).
        for pass in 0..2u32 {
            let (start, end) = if pass == 0 {
                (hint_sector, self.sectors_per_fat)
            } else {
                (0, core::cmp::min(hint_sector + 1, self.sectors_per_fat))
            };

            for i in start..end {
                let mut buf = [0; BUF_SIZE];
                self.device
                    .read_sector(self.fat_start_sector + i, &mut buf)?;

                let skip = if pass == 0 && i == hint_sector {
                    hint_offset
                } else {
                    0
                };

                for (id, bytes) in buf.chunks(4).enumerate().skip(skip) {
                    let entry = FatEntry::new_ref(bytes);
                    let cid = ENTRIES_PER_SECTOR as u32 * i + id as u32;
                    trace!("(cid: {:?}) Fat entry: {:?}", entry, cid);
                    if let FatEntry::Unused = entry {
                        debug!("Found an unused cluster with id: {}", cid);
                        return Ok(Some(ClusterId::new(cid)));
                    }
                }
            }
        }
        Ok(None)
    }

    /// Allocate a cluster for a new file.
    /// First find an empty cluster. Then set this cluster id as LastCluster.
    /// Updates the allocation hint so the next search starts after this cluster.
    pub(crate) fn allocate_cluster_new_entry(&self) -> Result<ClusterId> {
        let free_cluster_id = self
            .find_free_cluster()?
            .ok_or(VfatRsError::FreeClusterNotFound)?;
        let entry = self.new_last_cluster_fat_entry();
        info!("Found free cluster: {}", free_cluster_id);
        self.write_entry_in_vfat_table(free_cluster_id, entry)?;

        // Advance the hint past the just-allocated cluster.
        *self.last_alloc_hint.lock() = u32::from(free_cluster_id) + 1;

        Ok(free_cluster_id)
    }

    /// Finds a free clusters and updates the chain:
    ///  * previous cluster in the chain to point to the newly allocated one,
    /// * new clusterId added as final entry
    // TODO: invert writes, first update head, and then allocate the cluster.
    pub(crate) fn allocate_cluster_to_chain(&self, head: ClusterId) -> Result<ClusterId> {
        info!("Allocating cluster to chain: {}", head);
        debug!("Head cluster: {}", head);
        let tail_cluster_id = self.get_last_cluster_in_chain(head)?;
        debug!("Tail cluster: {}", tail_cluster_id);

        let free_cluster_id = self.allocate_cluster_new_entry()?;

        let updated_entry = FatEntry::from_chain(free_cluster_id);
        self.write_entry_in_vfat_table(tail_cluster_id, updated_entry)?;
        info!("Updated the entry");
        Ok(free_cluster_id)
    }
    fn write_entry_in_vfat_table(&self, cluster_id: ClusterId, entry: FatEntry) -> Result<()> {
        fat_table::set_fat_entry(self.device.clone(), cluster_id, entry)
    }

    fn get_last_cluster_in_chain(&self, starting: ClusterId) -> Result<ClusterId> {
        info!("Getting last cluster in the chain..");
        let mut last = starting;
        let mut iterations = 0;
        loop {
            ensure!(
                iterations < MAX_CLUSTER_CHAIN_LENGTH,
                error::FilesystemCorruptedSnafu {
                    reason: "Cluster chain exceeds maximum length (possible circular reference)"
                }
            );
            match fat_table::next_cluster(last, self.device.clone())? {
                Some(cluster_id) => {
                    last = cluster_id;
                    iterations += 1;
                }
                None => return Ok(last),
            }
        }
    }
    pub(crate) fn cluster_chain_writer(
        &self,
        cluster_id: ClusterId,
    ) -> cluster_writer::ClusterChainWriter {
        cluster_writer::ClusterChainWriter::new(self.clone(), cluster_id, SectorId::from(0), 0)
    }

    pub(crate) fn cluster_chain_reader(
        &self,
        cluster_id: ClusterId,
    ) -> cluster_reader::ClusterChainReader {
        cluster_reader::ClusterChainReader::new(self.device.clone(), cluster_id)
    }

    /// This will delete all the cluster chain starting from cluster_id.
    pub(crate) fn delete_fat_cluster_chain(&self, cluster_id: ClusterId) -> Result<()> {
        fat_table::delete_cluster_chain(cluster_id, self.device.clone())
    }

    /// Truncate a cluster chain, keeping `keep_count` clusters and freeing the rest.
    pub(crate) fn truncate_cluster_chain(&self, start: ClusterId, keep_count: u32) -> Result<()> {
        fat_table::truncate_cluster_chain(start, keep_count, self.device.clone())
    }

    /// Returns the number of bytes per cluster.
    pub(crate) fn bytes_per_cluster(&self) -> u32 {
        let device = self.device.clone();
        device.sectors_per_cluster * device.sector_size as u32
    }

    /// Write the current allocation hint back to the FSInfo sector on disk.
    ///
    /// This is advisory — the hint speeds up the next mount but correctness
    /// does not depend on it. Errors are silently ignored (best-effort).
    pub fn flush_fsinfo(&self) {
        let sector = match self.fsinfo_sector {
            Some(s) => s,
            None => return,
        };
        let hint = *self.last_alloc_hint.lock();
        // Write nxt_free (offset 492 = 4 + 480 + 4 + 4 = 492 bytes into the sector).
        let hint_bytes = hint.to_le_bytes();
        let _ = self
            .device
            .clone()
            .write_sector_offset(sector, 492, &hint_bytes);
    }

    /// Get a new DirectoryEntry from an absolute path.
    ///
    /// ## Safety:
    /// absolute_path should start with `/`.
    pub fn get_from_absolute_path(&mut self, absolute_path: PathBuf) -> Result<DirectoryEntry> {
        let lock = self.fs_lock.clone();
        let _guard = lock.read();
        self.get_from_absolute_path_unlocked(absolute_path)
    }

    /// Internal path resolution without acquiring the lock (for use by callers
    /// that already hold the lock).
    pub(crate) fn get_from_absolute_path_unlocked(
        &mut self,
        absolute_path: PathBuf,
    ) -> Result<DirectoryEntry> {
        ensure!(
            absolute_path.is_absolute(),
            error::PathNotAbsoluteSnafu {
                target: absolute_path.display().to_string()
            }
        );
        if absolute_path.iter().count() == 1 {
            return self.get_root_unlocked().map(From::from);
        }
        let mut path_iter = absolute_path.iter();
        let mut current_entry = DirectoryEntry::from(self.get_root_unlocked()?);
        path_iter.next();
        for sub_path in path_iter {
            let directory = current_entry.into_directory_or_not_found()?;
            let matches: Option<DirectoryEntry> = directory
                .contents_unlocked()?
                .into_iter()
                .filter(|entry| entry.metadata().name() == sub_path)
                .last();
            current_entry = matches.ok_or_else(|| VfatRsError::EntryNotFound {
                #[cfg(feature = "std")]
                target: sub_path.to_str().unwrap().into(),
                #[cfg(not(feature = "std"))]
                target: sub_path.into(),
            })?;
        }
        debug!("current entry: {:?}", current_entry);
        Ok(current_entry)
    }

    /// Returns `true` if the given path exists on this filesystem.
    pub fn path_exists(&mut self, path: PathBuf) -> Result<bool> {
        let lock = self.fs_lock.clone();
        let _guard = lock.read();
        let entry = self.get_from_absolute_path_unlocked(path).map(|_| true);
        match entry {
            Err(VfatRsError::EntryNotFound { .. }) => Ok(false),
            x => x,
        }
    }
    /// Returns the root directory of this filesystem.
    pub fn get_root(&mut self) -> Result<Directory> {
        let lock = self.fs_lock.clone();
        let _guard = lock.read();
        self.get_root_unlocked()
    }

    pub(crate) fn get_root_unlocked(&mut self) -> Result<Directory> {
        const UNKNOWN_ENTRIES: usize = 1;
        const BUF_SIZE: usize = UNKNOWN_ENTRIES * size_of::<UnknownDirectoryEntry>();
        let mut buf = [0; BUF_SIZE];
        let mut cluster_reader = self.cluster_chain_reader(self.root_cluster);
        let _ = cluster_reader.read(&mut buf)?;
        let unknown_entries: UnknownDirectoryEntry = buf.into();
        debug!("Unknown entries: {:?}", unknown_entries);
        let volume_id = VfatDirectoryEntry::from(unknown_entries)
            .into_regular()
            .filter(|regular| regular.is_volume_id())
            .ok_or_else(|| {
                crate::io::Error::new(crate::io::ErrorKind::NotFound, "Volume id not found?!")
            })?;

        let metadata = Metadata::new(
            volume_id.creation_time,
            volume_id.last_modification_time,
            "/",
            size_of::<RegularDirectoryEntry>() as u32,
            PathBuf::from("/"),
            self.root_cluster,
            PathBuf::from(""),
            Attributes::new_directory(),
        );
        Ok(Directory::new(self.clone(), metadata))
    }
}

#[cfg(test)]
mod test {
    use alloc::format;
    use alloc::sync::Arc;
    use alloc::vec::Vec;
    use spin::mutex::SpinMutex;
    use spin::rwlock::RwLock;

    use crate::fat_table::FAT_ENTRY_SIZE;
    use crate::io::Write;
    use crate::{
        BlockDevice, CachedPartition, ClusterId, Result, SectorId, TimeManagerNoop, VfatFS,
    };

    pub struct ArrayBackedBlockDevice {
        pub arr: Vec<u8>,
        pub read_iteration: usize,
    }

    impl BlockDevice for ArrayBackedBlockDevice {
        fn read_sector(&mut self, sector: SectorId, buf: &mut [u8]) -> Result<usize> {
            self.read_sector_offset(sector, 0, buf)
        }

        fn read_sector_offset(
            &mut self,
            _sector: SectorId,
            _offset: usize,
            mut buf: &mut [u8],
        ) -> Result<usize> {
            let ret = buf.write(&self.arr[self.read_iteration..512]);
            self.read_iteration += 1;
            ret.map_err(Into::into)
        }

        fn write_sector_offset(
            &mut self,
            _sector: SectorId,
            _offset: usize,
            _buf: &[u8],
        ) -> Result<usize> {
            unreachable!()
        }

        fn get_canonical_name() -> &'static str
        where
            Self: Sized,
        {
            "ArrayBackedBlockDevice"
        }
    }

    /// Mock device that simulates a circular cluster chain for testing loop detection.
    /// Chain: 2 → 3 → 4 → 2 (circular)
    pub struct CircularChainDevice;

    impl BlockDevice for CircularChainDevice {
        fn read_sector(&mut self, sector: SectorId, buf: &mut [u8]) -> Result<usize> {
            self.read_sector_offset(sector, 0, buf)
        }

        fn read_sector_offset(
            &mut self,
            _sector: SectorId,
            _offset: usize,
            buf: &mut [u8],
        ) -> Result<usize> {
            // Simulate FAT entries: cluster 2→3, 3→4, 4→2 (circular)
            // For simplicity, assume all reads get cluster 3 (next in chain)
            // In reality this would need to check the cluster being read
            buf[0..4].copy_from_slice(&3u32.to_le_bytes());
            Ok(4)
        }

        fn write_sector_offset(
            &mut self,
            _sector: SectorId,
            _offset: usize,
            _buf: &[u8],
        ) -> Result<usize> {
            unreachable!()
        }

        fn get_canonical_name() -> &'static str
        where
            Self: Sized,
        {
            "CircularChainDevice"
        }
    }

    #[test]
    fn test_circular_cluster_chain_detection() {
        let dev = CircularChainDevice;
        let vfat = VfatFS {
            device: Arc::new(CachedPartition::new(
                dev,
                512,
                SectorId(1),
                1,
                SectorId(100),
                2,
                50,
            )),
            fat_start_sector: SectorId(1),
            sectors_per_fat: 50,
            root_cluster: ClusterId::new(2),
            eoc_marker: Default::default(),
            time_manager: TimeManagerNoop::new_arc(),
            fs_lock: Arc::new(RwLock::new(())),
            last_alloc_hint: Arc::new(SpinMutex::new(2)),
            fsinfo_sector: None,
        };

        // Attempt to traverse the circular chain - should return error, not hang
        let result = vfat.get_last_cluster_in_chain(ClusterId::new(2));

        assert!(result.is_err(), "Expected error for circular cluster chain");
        let err = result.unwrap_err();
        assert!(
            format!("{}", err).contains("Filesystem corruption"),
            "Error should indicate filesystem corruption, got: {}",
            err
        );
    }

    #[test]
    fn test_find_next_free() {
        let mut ret = Vec::new();
        // Reserved entry:
        ret.extend_from_slice(&[0x01; FAT_ENTRY_SIZE]);
        // Free entry:
        ret.extend_from_slice(&[0x00; FAT_ENTRY_SIZE]);

        // Complete the sector:
        ret.extend_from_slice(&[0x01; 512 - (FAT_ENTRY_SIZE * 2)]);

        let dev = ArrayBackedBlockDevice {
            arr: ret,
            read_iteration: 0,
        };
        let sector_size = 1;
        let fat_start_sector = SectorId(0);
        let sectors_per_cluster = 1;
        let data_start_sector = SectorId(2);
        let fat_amount = 2;
        let sectors_per_fat = 1;
        let vfat = VfatFS {
            device: Arc::new(CachedPartition::new(
                dev,
                sector_size,
                fat_start_sector,
                sectors_per_cluster,
                data_start_sector,
                fat_amount,
                sectors_per_fat,
            )),
            fat_start_sector,
            sectors_per_fat,
            root_cluster: ClusterId::new(0),
            eoc_marker: Default::default(),
            time_manager: TimeManagerNoop::new_arc(),
            fs_lock: Arc::new(RwLock::new(())),
            last_alloc_hint: Arc::new(SpinMutex::new(0)),
            fsinfo_sector: None,
        };
        assert_eq!(
            vfat.find_free_cluster().unwrap().unwrap(),
            ClusterId::new(1)
        );
    }
}
