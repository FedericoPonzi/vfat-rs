use crate::io::{Read, Seek};
use crate::{BlockDevice, SectorId};
use log::debug;

/// FilebackedBlockDevice is an implementation of BlockDevice backed by
/// std::fs::File. It's a simple way to explore a vfat fs on a file.
pub struct FilebackedBlockDevice {
    pub image: std::fs::File,
}

impl BlockDevice for FilebackedBlockDevice {
    fn sector_size(&self) -> usize {
        512
    }

    fn read_sector_offset(
        &mut self,
        sector: SectorId,
        offset: usize,
        mut buf: &mut [u8],
    ) -> crate::Result<usize> {
        use core::cmp::min;
        let max_read = min(buf.len(), self.sector_size());
        let mut temp_buf = vec![0; max_read];
        let final_destination = sector.0 as u64 * self.sector_size() as u64 + offset as u64;
        debug!(
            "Sector: {}, offset: {}, finaldest: {}",
            sector.0 as u64 * self.sector_size() as u64,
            offset,
            final_destination
        );
        self.image
            .seek(std::io::SeekFrom::Start(final_destination))
            .expect("Impossible to seek to the sector");

        self.image
            .read_exact(temp_buf.as_mut_slice())
            .expect("Impossible to read from image");
        debug!("done reading read_sector_offset...");
        use crate::io::Write;
        buf.write(temp_buf.as_mut_slice()).map_err(Into::into)
    }

    fn write_sector_offset(
        &mut self,
        sector: SectorId,
        offset: usize,
        buf: &[u8],
    ) -> crate::Result<usize> {
        use std::io::Write;
        let final_destination = sector.0 as u64 * self.sector_size() as u64 + offset as u64;
        debug!(
            "Seeking to : sector: {}, sector_size: {}, offset: {}, final destination: {} ",
            sector,
            self.sector_size(),
            offset,
            final_destination
        );
        self.image
            .seek(std::io::SeekFrom::Start(final_destination))
            .expect("Error seek");
        debug!("Writing the buffer to the image..");
        self.image.write_all(buf).expect("Write sector");
        debug!("Written: {}", buf.len());
        self.image
            .flush()
            .map_err(|_| crate::io::ErrorKind::Other)?;
        Ok(buf.len())
    }

    fn get_canonical_name() -> &'static str
    where
        Self: Sized,
    {
        "FileBasedBlockDevice"
    }
}
