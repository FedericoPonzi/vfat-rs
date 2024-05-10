use alloc::boxed::Box;
use alloc::sync::Arc;

use log::info;
use spin::mutex::SpinMutex;

use crate::device::BlockDevice;
use crate::error::Result;
use crate::formats::cluster_id::ClusterId;
use crate::SectorId;

/// An interface to the underlaying Block Device.
/// It will cache entries, and help with reading and writing sectors.
pub(crate) struct CachedPartition {
    device: SpinMutex<Box<dyn BlockDevice>>,
    pub(crate) sector_size: usize,
    pub(crate) fat_start_sector: SectorId,
    /// How many sectors are mapped to a single cluster
    pub(crate) sectors_per_cluster: u32,
    /// First sector containing actual data - after all FAT tables.
    pub(crate) data_start_sector: SectorId,
}
impl CachedPartition {
    pub fn new<T>(
        device: T,
        sector_size: usize,
        fat_start_sector: SectorId,
        sectors_per_cluster: u32,
        data_start_sector: SectorId,
    ) -> Self
    where
        T: BlockDevice + 'static,
    {
        info!("Creating cached partition");
        Self {
            device: SpinMutex::new(Box::new(device)),
            sector_size,
            fat_start_sector,
            sectors_per_cluster,
            data_start_sector,
        }
    }

    pub fn flush(&self) -> Result<()> {
        todo!()
    }

    pub(crate) fn read_sector(&self, sector: SectorId, buf: &mut [u8]) -> Result<usize> {
        let mut dev_lock = self.device.lock();
        dev_lock.read_sector(sector, buf)
    }

    pub(crate) fn read_sector_offset(
        self: Arc<Self>,
        sector: SectorId,
        offset: usize,
        buf: &mut [u8],
    ) -> Result<usize> {
        let mut dev_lock = self.device.lock();
        dev_lock.read_sector_offset(sector, offset, buf)
    }
    #[allow(unused)]
    fn write_sector(self: Arc<Self>, sector: SectorId, buf: &[u8]) -> Result<usize> {
        let mut dev_lock = self.device.lock();
        dev_lock.write_sector(sector, buf)
    }

    pub(crate) fn write_sector_offset(
        self: Arc<Self>,
        sector: SectorId,
        offset: usize,
        buf: &[u8],
    ) -> Result<usize> {
        let mut dev_lock = self.device.lock();
        dev_lock.write_sector_offset(sector, offset, buf)
    }

    /// Converts a cluster (a FAT concept) to a sector (a BlockDevice concept).
    ///
    /// To do so, it uses some useful info from the BPB section.
    pub(crate) fn cluster_to_sector(&self, cluster: ClusterId) -> SectorId {
        let selected_sector = u32::from(cluster).saturating_sub(2) * self.sectors_per_cluster;
        let sect = self.data_start_sector.0 + selected_sector;
        SectorId(sect)
    }

    #[allow(unused)]
    fn get_canonical_name() -> &'static str
    where
        Self: Sized,
    {
        "CachePartition"
    }
}
/*
TODO:
impl Drop for CachedPartition {
    fn drop(&mut self) {
        unimplemented!()
    }
}
 */
