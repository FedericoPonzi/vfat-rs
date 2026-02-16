use crate::{error::Result, fat_table, ClusterId, SectorId, VfatFS};
use log::debug;

#[derive(Debug)]
pub(crate) struct ClusterChainWriter {
    vfat_fs: VfatFS,
    pub(crate) current_cluster: ClusterId,
    pub(crate) current_sector: SectorId,
    /// Offset in current_sector. In case buf.len()%sector_size != 0, this sector is not full read.
    /// The next read call will start from this offset.
    pub(crate) offset_byte_in_current_sector: usize,
}

impl ClusterChainWriter {
    pub(crate) fn new(
        vfat_fs: VfatFS,
        start_cluster: ClusterId,
        offset_sector_in_cluster: SectorId,
        offset_in_sector: usize,
    ) -> Self {
        let current_sector =
            vfat_fs.device.cluster_to_sector(start_cluster) + offset_sector_in_cluster;
        Self {
            current_cluster: start_cluster,
            offset_byte_in_current_sector: offset_in_sector,
            current_sector,
            vfat_fs,
        }
    }

    fn next_cluster_alloc(&mut self) -> Result<ClusterId> {
        let ret = fat_table::next_cluster(self.current_cluster, self.vfat_fs.device.clone())?;

        Ok(match ret {
            None => self
                .vfat_fs
                .allocate_cluster_to_chain(self.current_cluster)?,
            Some(r) => r,
        })
    }

    // If the offset is outside file, it will allocate clusters to accommodate requested seek size.
    /// Also: this allows seeking only forward, not backwards.
    pub fn seek(&mut self, offset: usize) -> Result<()> {
        debug!(
            "offset: {}, sector size: {} sectors per cluster: {}",
            offset, self.vfat_fs.device.sector_size, self.vfat_fs.device.sectors_per_cluster
        );

        // Calculate in which cluster this offset falls:
        let cluster_size =
            self.vfat_fs.device.sectors_per_cluster as usize * self.vfat_fs.device.sector_size;
        let cluster_offset = offset / cluster_size;
        debug!("Cluster offset: {}", cluster_offset);
        // Calculate in which sector this offset falls:
        let sector_offset = offset / self.vfat_fs.device.sector_size
            % self.vfat_fs.device.sectors_per_cluster as usize;

        // Finally, calculate the offset in the selected sector:
        let offset_in_sector = offset % self.vfat_fs.device.sector_size;

        for _ in 0..cluster_offset {
            self.current_cluster = self.next_cluster_alloc()?;
        }
        debug!(
            "offset in sector: {}, current cluster:{}",
            offset_in_sector, self.current_cluster
        );
        self.current_sector = self.vfat_fs.device.cluster_to_sector(self.current_cluster)
            + SectorId(sector_offset as u32);
        self.offset_byte_in_current_sector = offset_in_sector;

        Ok(())
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if buf.is_empty() {
            return Ok(0);
        }

        assert_ne!(
            self.current_cluster,
            ClusterId::new(0),
            "current cluster is ClusterId(0)."
        );

        let mut amount = 0;
        while amount < buf.len() {
            let current_amount_written = self.write_cluster(&buf[amount..])?;
            amount += current_amount_written;
            if current_amount_written == 0 {
                self.current_cluster = self.next_cluster_alloc()?;
                self.current_sector = self.vfat_fs.device.cluster_to_sector(self.current_cluster);
                self.offset_byte_in_current_sector = 0;
            }
        }
        debug!("CWW: Amount written: {}", amount);
        Ok(amount)
    }

    fn write_cluster(&mut self, buf: &[u8]) -> Result<usize> {
        if self.cluster_is_over() || buf.is_empty() {
            return Ok(0);
        }
        let mut total = 0;
        while total < buf.len() && !self.cluster_is_over() {
            let space_left_in_current_sector =
                self.vfat_fs.device.sector_size - self.offset_byte_in_current_sector;
            let amount = self.vfat_fs.device.clone().write_sector_offset(
                self.current_sector,
                self.offset_byte_in_current_sector,
                &buf[total..core::cmp::min(total + space_left_in_current_sector, buf.len())],
            )?;
            total += amount;
            self.offset_byte_in_current_sector += amount;
            assert!(self.offset_byte_in_current_sector <= self.vfat_fs.device.sector_size);

            if self.offset_byte_in_current_sector == self.vfat_fs.device.sector_size {
                // Sector is finished, let's go to the next one
                self.current_sector = self.current_sector + 1;
                self.offset_byte_in_current_sector = 0;
            }
        }
        Ok(total)
    }

    fn cluster_is_over(&self) -> bool {
        let cluster_start = self.vfat_fs.device.cluster_to_sector(self.current_cluster);
        let final_sector = SectorId(self.vfat_fs.device.sectors_per_cluster) + cluster_start;
        self.current_sector >= final_sector
    }
}
