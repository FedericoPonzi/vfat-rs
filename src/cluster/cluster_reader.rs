use crate::cache::CachedPartition;
use crate::{fat_table, ArcMutex, ClusterId, Result, SectorId};

/// this implements and encapsulates the logic needed to traverse
/// cluster chains, by reading the FAT table.
pub(crate) struct ClusterChainReader {
    device: ArcMutex<CachedPartition>,
    current_cluster: Option<ClusterId>,
    current_sector: SectorId,
    /// Offset in current_sector. In case buf.len()%sector_size != 0, this sector is not full read.
    /// The next read call will start from this offset.
    offset_byte_in_current_sector: usize,
}
impl ClusterChainReader {
    pub(crate) fn new(device: ArcMutex<CachedPartition>, start_cluster: ClusterId) -> Self {
        let current_sector = device.cluster_to_sector(start_cluster);

        Self {
            current_cluster: Some(start_cluster),
            offset_byte_in_current_sector: 0,
            current_sector,
            device,
        }
    }
    fn next_cluster(&self) -> Result<Option<ClusterId>> {
        if self.current_cluster.is_none() {
            return Ok(None);
        }
        fat_table::next_cluster(self.current_cluster.unwrap(), self.device.clone())
    }

    /// Assumptions: offset less then this object's size.
    /// Also: this allows seeking only forward, not backwards.
    pub fn seek(&mut self, offset: usize) -> Result<()> {
        // Calculate in which cluster this offset falls:
        let cluster_size = self.device.sectors_per_cluster as usize * self.device.sector_size;
        let cluster_offset = offset / cluster_size;

        // Calculate in which sector this offset falls:
        let sector_offset =
            offset / self.device.sector_size % self.device.sectors_per_cluster as usize;

        // Finally, calculate the offset in the selected sector:
        let offset_in_sector = offset % self.device.sector_size;
        /*info!(
            "Offset: {}, cluster_offset: {}, sector offset: {}, offset in sector: {}",
            offset, cluster_offset, sector_offset, offset_in_sector
        );*/
        for _ in 0..cluster_offset {
            self.current_cluster = self.next_cluster()?;
        }
        let start_sector = self.device.cluster_to_sector(self.current_cluster.unwrap())
            + SectorId(sector_offset as u32);
        self.current_sector = start_sector;
        self.offset_byte_in_current_sector = offset_in_sector;

        Ok(())
    }

    pub(crate) fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if self.current_cluster.is_none() || buf.is_empty() {
            return Ok(0);
        }

        let mut amount = 0;
        while amount < buf.len() && self.current_cluster.is_some() {
            let current_amount_read = self.read_cluster(&mut buf[amount..])?;
            amount += current_amount_read;
            if current_amount_read == 0 {
                self.current_cluster = self.next_cluster()?;
                if let Some(cluster) = self.current_cluster {
                    self.current_sector = self.device.cluster_to_sector(cluster);
                    self.offset_byte_in_current_sector = 0;
                }
            }
        }
        /*debug!(
            "CRR completed, red<buf: {}, is some: {}",
            amount < buf.len(),
            self.current_cluster.is_some()
        );*/

        Ok(amount)
    }

    /// A reader for the content of a single FAT cluster
    /// It reads from the beginning of the cluster.
    /// It's not thread-safe and should not be shared.
    /// Given a buffer buf, it will try to read as much sectors as it can in order to fill the buffer.
    fn read_cluster(&mut self, buf: &mut [u8]) -> Result<usize> {
        if self.cluster_is_over() || buf.is_empty() {
            return Ok(0);
        }

        let mut total = 0;
        let buf_len = buf.len();
        // Until buffer is full or I have read the whole cluster:
        while total < buf_len && !self.cluster_is_over() {
            let space_left_in_current_sector =
                self.device.sector_size - self.offset_byte_in_current_sector;
            let amount = self.device.clone().read_sector_offset(
                self.current_sector,
                self.offset_byte_in_current_sector,
                &mut buf[total..core::cmp::min(buf_len, space_left_in_current_sector)],
            )?;
            total += amount;
            self.offset_byte_in_current_sector += amount;
            assert!(self.offset_byte_in_current_sector <= self.device.sector_size);

            if self.offset_byte_in_current_sector == self.device.sector_size {
                self.current_sector = self.current_sector + 1;
                self.offset_byte_in_current_sector = 0;
            }
        }

        Ok(total)
    }
    fn cluster_is_over(&self) -> bool {
        let cluster_start = self.device.cluster_to_sector(self.current_cluster.unwrap());
        let final_sector = SectorId(self.device.sectors_per_cluster) + cluster_start;
        self.current_sector >= final_sector
    }
}
