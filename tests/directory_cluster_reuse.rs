//! Hermetic regression test for directory corruption on cluster reuse.
//!
//! When a new directory is created, its first cluster must be a valid *empty*
//! directory: the `.` and `..` entries followed by an end-of-entries (0x00)
//! marker for the remainder of the cluster. On a freshly formatted filesystem
//! free clusters are already zeroed, so writing only `.`/`..` happens to work.
//! But once the disk fills up, the allocator wraps around and hands out
//! previously used (and therefore non-zero) clusters. If the new directory's
//! cluster is not fully zero-initialised, the stale bytes past `..` get parsed
//! as bogus directory entries (the user observed garbage / "japanese" names).
//!
//! This test reproduces that scenario entirely in memory (no `mkfs.fat`, mount,
//! or sudo): it fills the volume to force cluster reuse and asserts that a
//! freshly created directory exposes no entries other than `.` and `..`.

use std::io::Cursor;
use std::sync::{Arc, Mutex};

use vfat_rs::{BlockDevice, SectorId, VfatFS, VfatMetadataTrait};

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

/// A newly created directory must be empty even when its cluster is a recycled
/// one that still holds stale directory data from a previous file.
#[test]
fn new_directory_on_recycled_cluster_is_empty() {
    let mut fs = fresh_fat32_fs(34);
    let mut root = fs.get_root().unwrap();

    // Fill the volume with a file made of an easily-recognisable byte pattern.
    // 0x5A ('Z') decodes to plausible-looking short-name directory entries, so a
    // missing zero-init would surface as bogus "ZZZZZZZZ.ZZZ" entries.
    let mut filler = root.create_file("filler.bin".to_string()).unwrap();
    let chunk = vec![0x5Au8; 64 * 1024];
    loop {
        match filler.write(&chunk) {
            Ok(0) => break,
            Ok(_) => {}
            Err(_) => break, // FreeClusterNotFound: disk full
        }
    }

    // Free the clusters again so the next allocation wraps around and reuses
    // them (now carrying stale 0x5A data).
    root.delete("filler.bin".to_string()).unwrap();

    // Create a fresh directory; it should land on a recycled, dirty cluster.
    let fresh = root.create_directory("fresh".to_string()).unwrap();
    let stale: Vec<String> = fresh
        .contents()
        .unwrap()
        .into_iter()
        .map(|e| e.name().to_string())
        .filter(|name| name != "." && name != "..")
        .collect();

    assert!(
        stale.is_empty(),
        "freshly created directory exposed stale entries: {stale:?}"
    );
}
