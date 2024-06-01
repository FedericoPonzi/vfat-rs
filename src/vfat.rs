use alloc::sync::Arc;
use core::{fmt, mem};

use binrw::io::Cursor;
use binrw::BinReaderExt;
use log::{debug, info, trace};
use snafu::ensure;

use crate::alloc::string::ToString;
use crate::cluster::{cluster_reader, cluster_writer};
use crate::fat_table::FatEntry;
use crate::fat_table::FAT_ENTRY_SIZE;
use crate::formats::extended_bios_parameter_block::FullExtendedBIOSParameterBlock;
use crate::{error, Result};
use crate::{
    fat_table, ArcMutex, Attributes, BlockDevice, CachedPartition, ClusterId, Directory, Metadata,
    RegularDirectoryEntry, SectorId, UnknownDirectoryEntry, VfatDirectoryEntry, VfatEntry,
    VfatRsError, EBPF_VFAT_MAGIC, EBPF_VFAT_MAGIC_ALT,
};
use crate::{PathBuf, TimeManagerTrait};

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
}

impl fmt::Debug for VfatFS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VfatFilesystem")
    }
}

impl VfatFS {
    #[cfg(not(feature = "std"))]
    pub fn new<B: BlockDevice + 'static>(
        device: B,
        // time_manager: T,
        partition_start_sector: u32,
    ) -> Result<Self> {
        let no_op = crate::traits::TimeManagerNoop::new();
        Self::new_tm(device, partition_start_sector, no_op)
    }

    #[cfg(feature = "std")]
    // chronos will be used as a time manager.
    pub fn new<B: BlockDevice + 'static>(device: B, partition_start_sector: u32) -> Result<Self> {
        let chronos_tm = crate::traits::TimeManagerChronos::new();
        Self::new_tm(device, partition_start_sector, chronos_tm)
    }

    pub fn new_tm<B: BlockDevice + 'static>(
        mut device: B,
        partition_start_sector: u32,
        time_manager: impl TimeManagerTrait + 'static,
    ) -> Result<Self> {
        let time_manager = Arc::new(time_manager);
        let full_ebpb = Self::read_fullebpb(&mut device, partition_start_sector)?;
        Self::new_with_ebpb(device, partition_start_sector, full_ebpb, time_manager)
    }

    pub fn read_fullebpb<B: BlockDevice + 'static>(
        device: &mut B,
        start_sector: u32,
    ) -> Result<FullExtendedBIOSParameterBlock> {
        let mut buff = [0u8; 512];
        device.read_sector(start_sector.into(), &mut buff)?;
        Ok(Cursor::new(&buff).read_le()?)
    }

    /// start_sector: Partition's start sector, or "Entry Offset Sector".
    fn new_with_ebpb<B: BlockDevice + 'static>(
        mut device: B,
        partition_start_sector: u32,
        full_ebpb: FullExtendedBIOSParameterBlock,
        time_manager: Arc<dyn TimeManagerTrait>,
    ) -> Result<Self> {
        let fat_start_sector =
            (partition_start_sector + full_ebpb.bpb.reserved_sectors as u32).into();
        let fats_total_size = full_ebpb.extended.sectors_per_fat * full_ebpb.bpb.fat_amount as u32;
        let data_start_sector =
            fat_start_sector + fats_total_size + full_ebpb.sectors_occupied_by_all_fats();
        let data_start_sector = SectorId(data_start_sector);

        let sectors_per_cluster = full_ebpb.bpb.sectors_per_cluster as u32;
        let root_cluster = ClusterId::new(full_ebpb.extended.root_cluster);
        let eoc_marker = Self::read_end_of_chain_marker(&mut device, fat_start_sector)?;
        let sector_size = device.sector_size();
        let cached_partition = CachedPartition::new(
            device,
            sector_size,
            fat_start_sector,
            sectors_per_cluster,
            data_start_sector,
        );
        let sectors_per_fat = full_ebpb.extended.sectors_per_fat;
        if full_ebpb.extended.signature != EBPF_VFAT_MAGIC
            && full_ebpb.extended.signature != EBPF_VFAT_MAGIC_ALT
        {
            return Err(VfatRsError::InvalidVfat {
                target: full_ebpb.extended.signature,
            });
        }
        Ok(VfatFS {
            device: Arc::new(cached_partition),
            fat_start_sector,
            root_cluster,
            eoc_marker,
            sectors_per_fat,
            time_manager,
        })
    }

    fn read_end_of_chain_marker<B>(device: &mut B, fat_start_sector: SectorId) -> Result<FatEntry>
    where
        B: BlockDevice,
    {
        const ENTRIES_BUF_SIZE: usize = 1;
        const BUF_SIZE: usize = FAT_ENTRY_SIZE * ENTRIES_BUF_SIZE;
        let mut buf = [0; BUF_SIZE];
        device.read_sector(fat_start_sector, &mut buf).unwrap();
        let raw_entry = FatEntry::from(buf);
        info!("End of chain marker: {:?}", raw_entry);
        Ok(raw_entry)
    }

    fn new_last_cluster_fat_entry(&self) -> FatEntry {
        // Last cluster is initialized with the eoc_marker
        FatEntry::LastCluster(self.eoc_marker.into())
    }

    /// Find next free cluster
    pub(crate) fn find_free_cluster(&self) -> Result<Option<ClusterId>> {
        info!("Starting find free cluster routine");
        // TODO: assumes sectors size.
        const ENTRIES_BUF_SIZE: usize = 512 / FAT_ENTRY_SIZE;
        const BUF_SIZE: usize = FAT_ENTRY_SIZE * ENTRIES_BUF_SIZE;
        // Iterate on each sector.
        for i in 0..self.sectors_per_fat {
            let mut buf = [0; BUF_SIZE];
            info!("reading sector: {}/{}", i, self.sectors_per_fat);
            self.device
                .read_sector(SectorId(self.fat_start_sector + i), &mut buf)
                .unwrap();
            let mut fat_entries = [FatEntry::default(); ENTRIES_BUF_SIZE];

            for (i, bytes) in buf.chunks(4).enumerate() {
                fat_entries[i] = FatEntry::new_ref(bytes);
            }

            for (id, fat_entry) in fat_entries.into_iter().enumerate() {
                let cid = (ENTRIES_BUF_SIZE as u32 * i) + id as u32;
                trace!("(cid: {:?}) Fat entry: {:?}", fat_entry, cid);
                if let FatEntry::Unused = fat_entry {
                    debug!("Found an unused cluster with id: {}", cid);
                    return Ok(Some(ClusterId::new(cid)));
                }
            }
        }
        Ok(None)
    }

    /// Allocate a cluster for a new file.
    /// First find an empty cluster. Then set this cluster id as LastCluster
    pub(crate) fn allocate_cluster_new_entry(&self) -> Result<ClusterId> {
        let free_cluster_id = self
            .find_free_cluster()?
            .ok_or(VfatRsError::FreeClusterNotFound)?;
        let entry = self.new_last_cluster_fat_entry();
        info!("Found free cluster: {}", free_cluster_id);
        self.write_entry_in_vfat_table(free_cluster_id, entry)?;
        Ok(free_cluster_id)
    }

    /// Finds a free clusters and updates the chain:
    ///  * previous cluster in the chain to point to the newly allocated one,
    /// * new clusterId added as final entry
    /// TODO: invert writes, first update head, and then allocate the cluster.
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
        loop {
            match fat_table::next_cluster(last, self.device.clone())? {
                Some(cluster_id) => last = cluster_id,
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

    /// p should start with `/`.
    /// Test with a path to a file, test with a path to root.
    pub fn get_from_absolute_path(&mut self, absolute_path: PathBuf) -> Result<VfatEntry> {
        ensure!(
            absolute_path.is_absolute(),
            error::PathNotAbsoluteSnafu {
                target: absolute_path.display().to_string()
            }
        );
        if absolute_path == PathBuf::from("/") {
            return self.get_root().map(From::from);
        }
        let mut path_iter = absolute_path.iter();
        let mut current_entry = VfatEntry::from(self.get_root()?);
        path_iter.next();
        for sub_path in path_iter {
            let directory = current_entry.into_directory_or_not_found()?;
            let directory_iter = directory.iter()?;
            let matches: Option<VfatEntry> = directory_iter
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

    pub fn path_exists(&mut self, path: PathBuf) -> Result<bool> {
        let entry = self.get_from_absolute_path(path).map(|_| true);
        match entry {
            Err(VfatRsError::EntryNotFound { .. }) => Ok(false),
            x => x,
        }
    }
    pub fn get_root(&mut self) -> Result<Directory> {
        const UNKNOWN_ENTRIES: usize = 1;
        const BUF_SIZE: usize = UNKNOWN_ENTRIES * mem::size_of::<UnknownDirectoryEntry>();
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
            mem::size_of::<RegularDirectoryEntry>() as u32,
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
    use alloc::sync::Arc;
    use alloc::vec::Vec;

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
        let vfat = VfatFS {
            device: Arc::new(CachedPartition::new(
                dev,
                sector_size,
                fat_start_sector,
                sectors_per_cluster,
                data_start_sector,
            )),
            fat_start_sector,
            sectors_per_fat: 1,
            root_cluster: ClusterId::new(0),
            eoc_marker: Default::default(),
            time_manager: TimeManagerNoop::new_arc(),
        };
        assert_eq!(
            vfat.find_free_cluster().unwrap().unwrap(),
            ClusterId::new(1)
        );
    }
}
