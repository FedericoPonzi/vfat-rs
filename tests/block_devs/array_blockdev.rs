use vfat_rs::BlockDevice;
use vfat_rs::SectorId;
use vfat_rs::io::Write;

//noinspection RsExternalLinter
pub struct ArrayBackedBlockDevice {
    pub arr: Vec<u8>,
    pub read_iteration: usize,
}

impl BlockDevice for ArrayBackedBlockDevice {
    fn read_sector(&mut self, sector: SectorId, buf: &mut [u8]) -> vfat_rs::Result<usize> {
        self.read_sector_offset(sector, 0, buf)
    }

    fn read_sector_offset(
        &mut self,
        _sector: SectorId,
        _offset: usize,
        mut buf: &mut [u8],
    ) -> vfat_rs::Result<usize> {
        let ret = buf.write(&self.arr[self.read_iteration..512]);
        self.read_iteration += 1;
        ret.map_err(Into::into)
    }

    fn write_sector_offset(
        &mut self,
        _sector: SectorId,
        _offset: usize,
        _buf: &[u8],
    ) -> vfat_rs::Result<usize> {
        todo!()
    }

    fn get_canonical_name() -> &'static str
    where
        Self: Sized,
    {
        "ArrayBackedBlockDevice"
    }
}
