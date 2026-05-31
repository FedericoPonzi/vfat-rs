//! Hermetic regression tests for the sequential-read fast path in [`vfat_rs::File`].
//!
//! These tests do not need `mkfs.fat`, `mount`, or `sudo`: they format a small
//! FAT32 image entirely in memory with the `fatfs` crate and then drive it
//! through vfat-rs against an in-RAM [`BlockDevice`]. This makes them runnable in
//! any environment (including CI) and lets them exercise the warm
//! `ClusterChainReader` cache that keeps repeated sequential reads O(1) instead
//! of re-walking the cluster chain from the start on every call.

use std::io::Cursor;
use std::sync::{Arc, Mutex};

use vfat_rs::io::SeekFrom;
use vfat_rs::{BlockDevice, SectorId, VfatFS};

const SECTOR_SIZE: usize = 512;

/// An in-memory block device backing a FAT32 image.
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

/// Format a fresh FAT32 image of `size_mib` MiB in memory and return a VfatFS over
/// it. The FAT volume starts at sector 0 (no MBR), so the partition start sector
/// passed to `VfatFS::new` is 0.
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

/// A position-dependent byte pattern: deliberately coprime with the sector and
/// cluster sizes so any off-by-a-cluster / off-by-a-sector seek bug shows up.
fn pattern_byte(offset: usize) -> u8 {
    (offset % 251) as u8
}

/// Reading a large file back in small chunks must reuse the warm
/// `ClusterChainReader` (no re-walk from the start each call) and stay correct.
#[test]
fn sequential_chunked_read_roundtrips() {
    let mut fs = fresh_fat32_fs(48);
    let mut root = fs.get_root().unwrap();
    let mut file = root.create_file("read.bin".to_string()).unwrap();

    let total = 1024 * 1024;
    let data: Vec<u8> = (0..total).map(pattern_byte).collect();
    file.write(&data).unwrap();

    // Read back in small chunks without seeking between reads: this exercises the
    // warm-reader reuse path (each read continues exactly where the last stopped).
    file.seek(SeekFrom::Start(0)).unwrap();
    let chunk = 1000; // deliberately not a sector/cluster multiple
    let mut got = vec![0u8; total];
    let mut read = 0;
    while read < total {
        let end = (read + chunk).min(total);
        let n = file.read(&mut got[read..end]).unwrap();
        assert!(n > 0, "unexpected EOF at offset {read}");
        read += n;
    }
    assert!(got == data, "content mismatch on chunked sequential read");
}

/// Interleaving reads with a forward seek must produce correct data (the warm
/// reader is invalidated by the seek and rebuilt at the new position).
#[test]
fn seek_then_read_is_correct() {
    let mut fs = fresh_fat32_fs(48);
    let mut root = fs.get_root().unwrap();
    let mut file = root.create_file("seekread.bin".to_string()).unwrap();

    let total = 256 * 1024;
    let data: Vec<u8> = (0..total).map(pattern_byte).collect();
    file.write(&data).unwrap();

    // Read a slice from the middle, then from near the end, then from the start.
    for &start in &[total / 2, total - 4096, 0, total / 3] {
        file.seek(SeekFrom::Start(start as u64)).unwrap();
        let len = 4096.min(total - start);
        let mut buf = vec![0u8; len];
        let mut read = 0;
        while read < len {
            let n = file.read(&mut buf[read..]).unwrap();
            assert!(n > 0);
            read += n;
        }
        assert!(
            buf == data[start..start + len],
            "content mismatch reading at offset {start}"
        );
    }
}
