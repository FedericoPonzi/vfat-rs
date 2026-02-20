use std::cmp::min;
use std::fs::File;
use std::io::SeekFrom;
use std::io::{Read, Seek};
use vfat_rs::io::Write;
use vfat_rs::BlockDevice;
use vfat_rs::SectorId;

pub struct FilebackedBlockDevice {
    pub image: File,
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
    ) -> vfat_rs::Result<usize> {
        let max_read = min(buf.len(), self.sector_size());
        let mut temp_buf = vec![0; max_read];
        let final_destination = sector.0 as u64 * self.sector_size() as u64 + offset as u64;
        self.image
            .seek(SeekFrom::Start(final_destination))
            .expect("Impossible to seek to the sector");
        self.image
            .read_exact(temp_buf.as_mut_slice())
            .expect("Impossible to read from image");
        buf.write(temp_buf.as_mut_slice()).map_err(Into::into)
    }

    fn write_sector_offset(
        &mut self,
        sector: SectorId,
        offset: usize,
        buf: &[u8],
    ) -> vfat_rs::Result<usize> {
        use std::io::Write;
        let final_destination = sector.0 as u64 * self.sector_size() as u64 + offset as u64;
        self.image
            .seek(SeekFrom::Start(final_destination))
            .expect("Error seek");
        self.image.write_all(buf).expect("Write sector");
        Ok(buf.len())
    }

    fn get_canonical_name() -> &'static str
    where
        Self: Sized,
    {
        "FileBasedBlockDevice"
    }
}
