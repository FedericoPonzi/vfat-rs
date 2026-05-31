//! Regression test: the cluster allocator must never hand out a cluster id past
//! the data area.
//!
//! A FAT is rounded up to a whole number of sectors, so its final sector can hold
//! "slack" entries that read as unused (0x00000000) yet map to sectors beyond the
//! end of the volume. Some formatters leave that slack zeroed. Allocating such a
//! cluster would make the writer address a sector past the volume and corrupt the
//! filesystem (this is one way an out-of-space copy could go wrong).
//!
//! This test deliberately zeroes the FAT slack of a freshly formatted image and
//! then fills the volume through a *strict* block device that refuses any write
//! beyond the real volume size. Filling must terminate with a clean
//! `FreeClusterNotFound` (ENOSPC), never an out-of-bounds device error.

use std::io::Cursor;
use std::sync::{Arc, Mutex};

use vfat_rs::{BlockDevice, SectorId, VfatFS, VfatRsError};

const SECTOR_SIZE: usize = 512;

/// A fixed-size in-memory device that errors on any access beyond the volume,
/// instead of silently growing. An out-of-bounds write surfaces as a
/// `FilesystemCorrupted` sentinel so the test can distinguish it from ENOSPC.
#[derive(Clone)]
struct StrictBlockDevice(Arc<Mutex<Vec<u8>>>);

impl BlockDevice for StrictBlockDevice {
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
            return Err(VfatRsError::FilesystemCorrupted {
                reason: "write beyond end of volume (allocator escaped the data area)",
            });
        }
        data[start..start + buf.len()].copy_from_slice(buf);
        Ok(buf.len())
    }
}

fn le_u16(b: &[u8], o: usize) -> u16 {
    u16::from_le_bytes([b[o], b[o + 1]])
}
fn le_u32(b: &[u8], o: usize) -> u32 {
    u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]])
}

#[test]
fn allocator_never_escapes_data_area_when_fat_slack_is_zeroed() {
    let size = 34 * 1024 * 1024;
    let mut image = vec![0u8; size];
    {
        let cursor = Cursor::new(&mut image[..]);
        let options = fatfs::FormatVolumeOptions::new()
            .fat_type(fatfs::FatType::Fat32)
            .volume_label(*b"VFATRSTEST ");
        fatfs::format_volume(cursor, options).expect("format FAT32 image");
    }

    // Read enough of the BPB to locate the FATs, then determine the number of
    // addressable data clusters via VfatFS and zero out every FAT slack entry.
    let reserved_sectors = le_u16(&image, 14) as usize;
    let num_fats = image[16] as usize;
    let sectors_per_fat = le_u32(&image, 36) as usize;

    let total_clusters = {
        let dev = StrictBlockDevice(Arc::new(Mutex::new(image.clone())));
        VfatFS::new(dev, 0).unwrap().cluster_count()
    };

    // Valid data cluster ids are 2..2+total_clusters; everything at or beyond
    // that index in the FAT is slack. Zero it in every FAT copy.
    let entries_per_fat = sectors_per_fat * SECTOR_SIZE / 4;
    let first_slack = (2 + total_clusters) as usize;
    for fat in 0..num_fats {
        let fat_start = (reserved_sectors + fat * sectors_per_fat) * SECTOR_SIZE;
        for cid in first_slack..entries_per_fat {
            let off = fat_start + cid * 4;
            image[off..off + 4].copy_from_slice(&[0, 0, 0, 0]);
        }
    }
    assert!(
        entries_per_fat > first_slack,
        "test image must actually have FAT slack to be meaningful"
    );

    // Now fill the volume completely and make sure we stop with ENOSPC, never an
    // out-of-bounds write.
    let mut fs = VfatFS::new(StrictBlockDevice(Arc::new(Mutex::new(image))), 0).unwrap();
    let mut root = fs.get_root().unwrap();
    let mut file = root.create_file("filler.bin".to_string()).unwrap();
    let chunk = vec![0x5Au8; 64 * 1024];

    loop {
        match file.write(&chunk) {
            Ok(0) => break,
            Ok(_) => {}
            Err(VfatRsError::FreeClusterNotFound) => break, // expected: disk full
            Err(other) => panic!("allocator escaped the data area: {other:?}"),
        }
    }
}
