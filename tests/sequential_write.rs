//! Hermetic regression tests for the sequential-write fast path in [`vfat_rs::File`].
//!
//! These tests do not need `mkfs.fat`, `mount`, or `sudo`: they format a small
//! FAT32 image entirely in memory with the `fatfs` crate and then drive it
//! through vfat-rs against an in-RAM [`BlockDevice`]. This makes them runnable in
//! any environment (including CI) and lets them exercise the warm
//! `ClusterChainWriter` cache that keeps repeated sequential writes O(1) instead
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

fn read_whole_file(file: &mut vfat_rs::File, size: usize) -> Vec<u8> {
    file.seek(SeekFrom::Start(0)).unwrap();
    let mut out = vec![0u8; size];
    let mut got = 0;
    while got < size {
        let n = file.read(&mut out[got..]).unwrap();
        assert!(n > 0, "unexpected EOF at {got}/{size}");
        got += n;
    }
    out
}

/// Writing a multi-megabyte file in many small chunks through a single handle
/// must reproduce the exact bytes on read-back. This is the path that reuses the
/// warm writer: with N chunks it would previously perform O(N^2) FAT walks.
#[test]
fn sequential_chunked_write_roundtrips() {
    // 64 MiB image -> FAT32 with 512-byte clusters, so an 8 MiB file spans
    // thousands of clusters and the per-write chain walk would dominate without
    // the warm-writer cache.
    let mut fs = fresh_fat32_fs(64);
    let mut root = fs.get_root().unwrap();
    let mut file = root.create_file("seq.bin".to_string()).unwrap();

    let total = 8 * 1024 * 1024;
    let chunk_size = 4096;
    let mut expected = Vec::with_capacity(total);
    let mut written = 0;
    while written < total {
        let chunk: Vec<u8> = (written..written + chunk_size).map(pattern_byte).collect();
        let n = file.write(&chunk).unwrap();
        assert_eq!(n, chunk_size, "short write at offset {written}");
        expected.extend_from_slice(&chunk);
        written += n;
    }

    assert_eq!(file.metadata().size(), total);
    let got = read_whole_file(&mut file, total);
    assert!(
        got == expected,
        "read-back content differs from what was written"
    );
}

/// Seeking backwards must invalidate the warm writer so a later overwrite lands
/// at the right place rather than continuing the previous sequential run.
#[test]
fn seek_backwards_then_overwrite_is_correct() {
    let mut fs = fresh_fat32_fs(48);
    let mut root = fs.get_root().unwrap();
    let mut file = root.create_file("seek.bin".to_string()).unwrap();

    let total = 1024 * 1024;
    let chunk_size = 4096;
    let mut expected: Vec<u8> = (0..total).map(pattern_byte).collect();

    // Sequential fill (uses the warm writer).
    let mut written = 0;
    while written < total {
        let n = file
            .write(&expected[written..written + chunk_size])
            .unwrap();
        assert_eq!(n, chunk_size);
        written += n;
    }

    // Overwrite a region in the middle after seeking backwards.
    let overwrite_at = 300 * 1024;
    let overwrite: Vec<u8> = (0..chunk_size).map(|i| (i % 97) as u8 ^ 0x5A).collect();
    file.seek(SeekFrom::Start(overwrite_at as u64)).unwrap();
    let n = file.write(&overwrite).unwrap();
    assert_eq!(n, chunk_size);
    expected[overwrite_at..overwrite_at + chunk_size].copy_from_slice(&overwrite);

    // Continue writing right after the overwrite (warm writer rebuilt at new pos).
    let cont: Vec<u8> = (0..chunk_size).map(|i| (i % 131) as u8).collect();
    let n = file.write(&cont).unwrap();
    assert_eq!(n, chunk_size);
    expected[overwrite_at + chunk_size..overwrite_at + 2 * chunk_size].copy_from_slice(&cont);

    let got = read_whole_file(&mut file, total);
    assert!(got == expected, "content mismatch after seek+overwrite");
}

/// Truncating frees clusters and must invalidate the warm writer; a subsequent
/// extending write must still produce correct data.
#[test]
fn truncate_then_extend_is_correct() {
    let mut fs = fresh_fat32_fs(48);
    let mut root = fs.get_root().unwrap();
    let mut file = root.create_file("trunc.bin".to_string()).unwrap();

    let total = 512 * 1024;
    let chunk_size = 4096;
    let full: Vec<u8> = (0..total).map(pattern_byte).collect();
    let mut written = 0;
    while written < total {
        let n = file.write(&full[written..written + chunk_size]).unwrap();
        assert_eq!(n, chunk_size);
        written += n;
    }

    let keep = 100 * 1024;
    file.truncate(keep as u32).unwrap();
    assert_eq!(file.metadata().size(), keep);

    // Re-extend by appending from the end.
    file.seek(SeekFrom::End(0)).unwrap();
    let tail: Vec<u8> = (0..chunk_size).map(|i| (i % 89) as u8).collect();
    let n = file.write(&tail).unwrap();
    assert_eq!(n, chunk_size);

    let mut expected = full[..keep].to_vec();
    expected.extend_from_slice(&tail);

    let got = read_whole_file(&mut file, keep + chunk_size);
    assert!(got == expected, "content mismatch after truncate+extend");
}
