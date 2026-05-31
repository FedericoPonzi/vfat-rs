//! Hermetic regression test for [`vfat_rs::File::set_timestamps`].
//!
//! Formats a small FAT32 image in memory with the `fatfs` crate (no `mkfs.fat`,
//! `mount`, or `sudo` required) and verifies that explicitly-set creation and
//! modification timestamps survive a round trip through the on-disk directory
//! entry.

use std::io::Cursor;
use std::sync::{Arc, Mutex};

use vfat_rs::{BlockDevice, SectorId, VfatFS, VfatTimestamp};

const SECTOR_SIZE: usize = 512;

#[derive(Clone)]
struct MemoryBlockDevice(Arc<Mutex<Vec<u8>>>);

impl BlockDevice for MemoryBlockDevice {
    fn read_sector_offset(
        &mut self,
        sector: SectorId,
        offset: usize,
        buf: &mut [u8],
    ) -> vfat_rs::Result<usize> {
        let data = self.0.lock().unwrap();
        let start = sector.0 as usize * SECTOR_SIZE + offset;
        let available = data.len().saturating_sub(start);
        let n = buf.len().min(available);
        buf[..n].copy_from_slice(&data[start..start + n]);
        Ok(n)
    }

    fn write_sector_offset(
        &mut self,
        sector: SectorId,
        offset: usize,
        buf: &[u8],
    ) -> vfat_rs::Result<usize> {
        let mut data = self.0.lock().unwrap();
        let start = sector.0 as usize * SECTOR_SIZE + offset;
        if start + buf.len() > data.len() {
            data.resize(start + buf.len(), 0);
        }
        data[start..start + buf.len()].copy_from_slice(buf);
        Ok(buf.len())
    }
}

fn fresh_fat32_fs(size_mib: usize) -> VfatFS {
    let mut image = vec![0u8; size_mib * 1024 * 1024];
    {
        let cursor = Cursor::new(&mut image[..]);
        let options = fatfs::FormatVolumeOptions::new()
            .fat_type(fatfs::FatType::Fat32)
            .volume_label(*b"VFATRSTEST ");
        fatfs::format_volume(cursor, options).expect("format FAT32 image");
    }
    let device = MemoryBlockDevice(Arc::new(Mutex::new(image)));
    VfatFS::new(device, 0).expect("open VfatFS")
}

/// Explicitly setting creation and modification timestamps must persist them to
/// the directory entry so a fresh lookup reads them back (within VFAT's 2-second
/// resolution).
#[test]
fn set_timestamps_round_trips_through_disk() {
    let mut fs = fresh_fat32_fs(48);
    let mut root = fs.get_root().unwrap();
    let _ = root.create_file("dated.txt".to_string()).unwrap();

    // 2007-06-15 12:30:20 UTC and 2021-04-23 08:15:00 UTC (chosen to round cleanly
    // to VFAT's 2-second resolution).
    let creation_unix: u64 = 1_181_910_620;
    let modification_unix: u64 = 1_619_165_700;
    let creation = VfatTimestamp::from(creation_unix);
    let modification = VfatTimestamp::from(modification_unix);

    {
        let mut file = fs
            .get_from_absolute_path("/dated.txt".into())
            .unwrap()
            .into_file()
            .unwrap();
        file.set_timestamps(Some(creation), Some(modification))
            .unwrap();
    }

    // Re-open from scratch: this re-parses the on-disk directory entry.
    let file = fs
        .get_from_absolute_path("/dated.txt".into())
        .unwrap()
        .into_file()
        .unwrap();

    assert_eq!(
        file.metadata().created().to_unix_timestamp(),
        creation.to_unix_timestamp(),
        "creation timestamp did not round-trip"
    );
    assert_eq!(
        file.metadata().modified().to_unix_timestamp(),
        modification.to_unix_timestamp(),
        "modification timestamp did not round-trip"
    );
}

/// A `None` argument must leave the corresponding timestamp untouched.
#[test]
fn set_timestamps_none_leaves_field_unchanged() {
    let mut fs = fresh_fat32_fs(48);
    let mut root = fs.get_root().unwrap();
    let _ = root.create_file("partial.txt".to_string()).unwrap();

    let original_creation = fs
        .get_from_absolute_path("/partial.txt".into())
        .unwrap()
        .into_file()
        .unwrap()
        .metadata()
        .created()
        .to_unix_timestamp();

    let new_modification = VfatTimestamp::from(1_619_165_700u64);
    {
        let mut file = fs
            .get_from_absolute_path("/partial.txt".into())
            .unwrap()
            .into_file()
            .unwrap();
        // Only update modification; leave creation alone.
        file.set_timestamps(None, Some(new_modification)).unwrap();
    }

    let file = fs
        .get_from_absolute_path("/partial.txt".into())
        .unwrap()
        .into_file()
        .unwrap();
    assert_eq!(
        file.metadata().created().to_unix_timestamp(),
        original_creation,
        "creation timestamp must be unchanged when None is passed"
    );
    assert_eq!(
        file.metadata().modified().to_unix_timestamp(),
        new_modification.to_unix_timestamp(),
        "modification timestamp should have been updated"
    );
}
