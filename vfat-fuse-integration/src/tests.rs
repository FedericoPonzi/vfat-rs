//! Unit/integration tests for the FUSE adapter logic.
//!
//! These tests do **not** mount a real FUSE filesystem (which would need root
//! privileges and a running kernel module). Instead they build a FAT32 image
//! with `mkfs.fat`, populate it through the vfat-rs API, and then drive the
//! adapter's [`Inner`] methods directly, asserting on the values that the FUSE
//! reply layer would forward to the kernel. The write-path tests additionally
//! read the data back to confirm it was persisted to the image.
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use std::process::Command;

use fuser::FileType;
use vfat_rs::{FilebackedBlockDevice, VfatFS};

use super::{detect_start_sector, Inner, ROOT_INODE};

const FILE_NAME: &str = "hello.txt";
const FILE_CONTENT: &[u8] = b"Hello, FUSE world!";
const DIR_NAME: &str = "subdir";
/// A 1 MiB-aligned first-partition offset, matching the repo's `tests/setup.sh`.
const PARTITION_START: u32 = 2048;
/// Size of the FAT volume created for tests.
const FS_SIZE_BYTES: u64 = 64 * 1024 * 1024;

/// Create a raw (non-partitioned) FAT32 image at `path` using `mkfs.fat`.
///
/// Returns `false` if `mkfs.fat` is not available, so callers can skip the
/// test gracefully on machines without dosfstools installed.
fn create_fat32_image(path: &Path) -> bool {
    // 64 MiB, expressed in 1K blocks as mkfs.fat expects.
    let blocks = (FS_SIZE_BYTES / 1024).to_string();
    let status = Command::new("mkfs.fat")
        .args(["-F", "32", "-n", "VFATFUSE", "-C", path.to_str().unwrap()])
        .arg(&blocks)
        .status();
    matches!(status, Ok(status) if status.success())
}

/// Create a partitioned image: an MBR at sector 0 referencing a single FAT32
/// partition whose volume starts at `start_sector`.
///
/// The FAT volume itself is produced with `mkfs.fat`; the MBR partition table
/// is written by hand so the test does not depend on `parted`. Returns `false`
/// if `mkfs.fat` is unavailable.
fn create_partitioned_fat32_image(path: &Path, start_sector: u32) -> bool {
    use std::io::{Read, Seek, SeekFrom, Write};

    let fs_path = path.with_extension("vol");
    if !create_fat32_image(&fs_path) {
        return false;
    }
    let mut volume = Vec::new();
    OpenOptions::new()
        .read(true)
        .open(&fs_path)
        .unwrap()
        .read_to_end(&mut volume)
        .unwrap();

    let offset = start_sector as u64 * vfat_rs::SECTOR_SIZE as u64;
    let total_sectors = (volume.len() / vfat_rs::SECTOR_SIZE) as u32;

    let mut image = OpenOptions::new()
        .create(true)
        .truncate(true)
        .read(true)
        .write(true)
        .open(path)
        .unwrap();
    image.set_len(offset + volume.len() as u64).unwrap();

    // Write the FAT volume at the partition offset.
    image.seek(SeekFrom::Start(offset)).unwrap();
    image.write_all(&volume).unwrap();

    // Write a single MBR partition entry (offset 0x1BE) plus the boot signature.
    let mut entry = [0u8; 16];
    entry[0] = 0x80; // bootable
    entry[4] = 0x0C; // partition type: FAT32 (LBA)
    entry[8..12].copy_from_slice(&start_sector.to_le_bytes());
    entry[12..16].copy_from_slice(&total_sectors.to_le_bytes());
    image.seek(SeekFrom::Start(0x1BE)).unwrap();
    image.write_all(&entry).unwrap();
    image.seek(SeekFrom::Start(510)).unwrap();
    image.write_all(&[0x55, 0xAA]).unwrap();
    image.flush().unwrap();
    true
}

/// Open `path` as a read/write file-backed block device.
fn open_device(path: &Path) -> FilebackedBlockDevice {
    FilebackedBlockDevice {
        image: OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
            .unwrap(),
    }
}

/// Populate a freshly opened filesystem with the fixtures used by the tests.
fn populate(fs: &mut VfatFS) {
    let mut root = fs.get_root().unwrap();
    let mut file = root.create_file(FILE_NAME.to_string()).unwrap();
    file.write(FILE_CONTENT).unwrap();
    root.create_directory(DIR_NAME.to_string()).unwrap();
}

/// Build a populated [`Inner`] over a fresh raw FAT32 image, or `None` to skip.
fn setup() -> Option<(Inner, tempfile::TempDir)> {
    let dir = tempfile::tempdir().unwrap();
    let image_path = dir.path().join("fat32.img");
    if !create_fat32_image(&image_path) {
        eprintln!("skipping test: mkfs.fat not available");
        return None;
    }

    // Raw mkfs.fat image: the FAT volume starts at sector 0.
    let mut fs = VfatFS::new(open_device(&image_path), 0).unwrap();
    populate(&mut fs);

    Some((Inner::new(fs), dir))
}

#[test]
fn root_getattr_is_directory() {
    let Some((mut inner, _guard)) = setup() else {
        return;
    };
    let attr = inner.getattr(ROOT_INODE).unwrap();
    assert_eq!(attr.kind, FileType::Directory);
    assert_eq!(attr.ino.0, ROOT_INODE);
}

#[test]
fn lookup_returns_file_attributes() {
    let Some((mut inner, _guard)) = setup() else {
        return;
    };
    let attr = inner.lookup(ROOT_INODE, FILE_NAME).unwrap();
    assert_eq!(attr.kind, FileType::RegularFile);
    assert_eq!(attr.size, FILE_CONTENT.len() as u64);
}

#[test]
fn lookup_missing_entry_is_error() {
    let Some((mut inner, _guard)) = setup() else {
        return;
    };
    assert!(inner.lookup(ROOT_INODE, "does-not-exist.txt").is_err());
}

#[test]
fn read_returns_file_content() {
    let Some((mut inner, _guard)) = setup() else {
        return;
    };
    let attr = inner.lookup(ROOT_INODE, FILE_NAME).unwrap();
    let data = inner
        .read(attr.ino.0, 0, FILE_CONTENT.len() as u32)
        .unwrap();
    assert_eq!(data, FILE_CONTENT);
}

#[test]
fn read_with_offset() {
    let Some((mut inner, _guard)) = setup() else {
        return;
    };
    let attr = inner.lookup(ROOT_INODE, FILE_NAME).unwrap();
    let data = inner.read(attr.ino.0, 7, 1024).unwrap();
    assert_eq!(data, &FILE_CONTENT[7..]);
}

#[test]
fn readdir_lists_entries() {
    let Some((mut inner, _guard)) = setup() else {
        return;
    };
    let entries = inner.readdir(ROOT_INODE).unwrap();
    let names: Vec<&str> = entries.iter().map(|(_, _, name)| name.as_str()).collect();
    assert!(names.contains(&"."));
    assert!(names.contains(&".."));
    assert!(names.contains(&FILE_NAME));
    assert!(names.contains(&DIR_NAME));

    let kind_of = |target: &str| {
        entries
            .iter()
            .find(|(_, _, name)| name == target)
            .map(|(_, kind, _)| *kind)
    };
    assert_eq!(kind_of(FILE_NAME), Some(FileType::RegularFile));
    assert_eq!(kind_of(DIR_NAME), Some(FileType::Directory));
}

#[test]
fn subdirectory_lookup_and_readdir() {
    let Some((mut inner, _guard)) = setup() else {
        return;
    };
    let dir_attr = inner.lookup(ROOT_INODE, DIR_NAME).unwrap();
    assert_eq!(dir_attr.kind, FileType::Directory);

    let entries = inner.readdir(dir_attr.ino.0).unwrap();
    let names: Vec<&str> = entries.iter().map(|(_, _, name)| name.as_str()).collect();
    assert!(names.contains(&"."));
    assert!(names.contains(&".."));

    // `..` of the subdirectory must point back at the root inode.
    let parent_ino = entries
        .iter()
        .find(|(_, _, name)| name == "..")
        .map(|(ino, _, _)| *ino)
        .unwrap();
    assert_eq!(parent_ino, ROOT_INODE);
}

#[test]
fn inode_numbers_are_stable() {
    let Some((mut inner, _guard)) = setup() else {
        return;
    };
    let first = inner.lookup(ROOT_INODE, FILE_NAME).unwrap().ino.0;
    let second = inner.lookup(ROOT_INODE, FILE_NAME).unwrap().ino.0;
    assert_eq!(first, second);
    assert_ne!(first, ROOT_INODE);
}

#[test]
fn path_resolution_round_trips() {
    let Some((mut inner, _guard)) = setup() else {
        return;
    };
    let ino = inner.lookup(ROOT_INODE, FILE_NAME).unwrap().ino.0;
    assert_eq!(
        inner.path_of(ino).unwrap(),
        PathBuf::from("/").join(FILE_NAME)
    );
}

#[test]
fn detect_raw_image_returns_sector_zero() {
    let dir = tempfile::tempdir().unwrap();
    let image_path = dir.path().join("raw.img");
    if !create_fat32_image(&image_path) {
        eprintln!("skipping test: mkfs.fat not available");
        return;
    }
    assert_eq!(detect_start_sector(&image_path).unwrap(), 0);
}

#[test]
fn detect_partitioned_image_returns_partition_start() {
    let dir = tempfile::tempdir().unwrap();
    let image_path = dir.path().join("part.img");
    if !create_partitioned_fat32_image(&image_path, PARTITION_START) {
        eprintln!("skipping test: mkfs.fat not available");
        return;
    }
    assert_eq!(detect_start_sector(&image_path).unwrap(), PARTITION_START);
}

#[test]
fn detect_missing_image_is_error() {
    let path = PathBuf::from("/nonexistent/definitely-not-here.img");
    assert!(detect_start_sector(&path).is_err());
}

/// End-to-end on a partitioned image: detect the partition, populate it through
/// the detected offset, and read it back through the adapter.
#[test]
fn partitioned_image_round_trips_through_adapter() {
    let dir = tempfile::tempdir().unwrap();
    let image_path = dir.path().join("part.img");
    if !create_partitioned_fat32_image(&image_path, PARTITION_START) {
        eprintln!("skipping test: mkfs.fat not available");
        return;
    }

    let start = detect_start_sector(&image_path).unwrap();
    assert_eq!(start, PARTITION_START);

    let mut fs = VfatFS::new(open_device(&image_path), start).unwrap();
    populate(&mut fs);
    let mut inner = Inner::new(fs);

    let attr = inner.lookup(ROOT_INODE, FILE_NAME).unwrap();
    assert_eq!(attr.kind, FileType::RegularFile);
    assert_eq!(attr.size, FILE_CONTENT.len() as u64);

    let data = inner
        .read(attr.ino.0, 0, FILE_CONTENT.len() as u32)
        .unwrap();
    assert_eq!(data, FILE_CONTENT);
}

// ---------------------------------------------------------------------------
// Write-path tests
// ---------------------------------------------------------------------------

#[test]
fn create_makes_empty_file() {
    let Some((mut inner, _guard)) = setup() else {
        return;
    };
    let attr = inner.create(ROOT_INODE, "new.txt").unwrap();
    assert_eq!(attr.kind, FileType::RegularFile);
    assert_eq!(attr.size, 0);

    // It is visible through lookup and readdir.
    assert!(inner.lookup(ROOT_INODE, "new.txt").is_ok());
    let names: Vec<String> = inner
        .readdir(ROOT_INODE)
        .unwrap()
        .into_iter()
        .map(|(_, _, name)| name)
        .collect();
    assert!(names.iter().any(|name| name == "new.txt"));
}

#[test]
fn write_then_read_round_trips() {
    let Some((mut inner, _guard)) = setup() else {
        return;
    };
    let ino = inner.create(ROOT_INODE, "w.txt").unwrap().ino.0;
    let payload = b"the quick brown fox";
    let written = inner.write(ino, 0, payload).unwrap();
    assert_eq!(written as usize, payload.len());

    assert_eq!(inner.getattr(ino).unwrap().size, payload.len() as u64);
    let data = inner.read(ino, 0, payload.len() as u32).unwrap();
    assert_eq!(data, payload);
}

#[test]
fn write_past_eof_zero_fills_gap() {
    let Some((mut inner, _guard)) = setup() else {
        return;
    };
    let ino = inner.create(ROOT_INODE, "sparse.txt").unwrap().ino.0;
    let payload = b"DATA";
    inner.write(ino, 100, payload).unwrap();

    assert_eq!(inner.getattr(ino).unwrap().size, 104);
    let data = inner.read(ino, 0, 104).unwrap();
    assert_eq!(&data[..100], &[0u8; 100]);
    assert_eq!(&data[100..], payload);
}

#[test]
fn setattr_truncate_shrinks_file() {
    let Some((mut inner, _guard)) = setup() else {
        return;
    };
    let attr = inner.lookup(ROOT_INODE, FILE_NAME).unwrap();
    let new_attr = inner.set_size(attr.ino.0, 5).unwrap();
    assert_eq!(new_attr.size, 5);

    assert_eq!(inner.getattr(attr.ino.0).unwrap().size, 5);
    let data = inner.read(attr.ino.0, 0, 64).unwrap();
    assert_eq!(data, &FILE_CONTENT[..5]);
}

#[test]
fn setattr_extend_zero_fills_file() {
    let Some((mut inner, _guard)) = setup() else {
        return;
    };
    let attr = inner.lookup(ROOT_INODE, FILE_NAME).unwrap();
    let target = FILE_CONTENT.len() as u64 + 10;
    inner.set_size(attr.ino.0, target).unwrap();

    assert_eq!(inner.getattr(attr.ino.0).unwrap().size, target);
    let data = inner.read(attr.ino.0, 0, target as u32).unwrap();
    assert_eq!(&data[..FILE_CONTENT.len()], FILE_CONTENT);
    assert_eq!(&data[FILE_CONTENT.len()..], &[0u8; 10]);
}

#[test]
fn mkdir_creates_directory() {
    let Some((mut inner, _guard)) = setup() else {
        return;
    };
    let attr = inner.mkdir(ROOT_INODE, "newdir").unwrap();
    assert_eq!(attr.kind, FileType::Directory);

    let looked_up = inner.lookup(ROOT_INODE, "newdir").unwrap();
    assert_eq!(looked_up.kind, FileType::Directory);
    let names: Vec<String> = inner
        .readdir(looked_up.ino.0)
        .unwrap()
        .into_iter()
        .map(|(_, _, name)| name)
        .collect();
    assert!(names.iter().any(|name| name == "."));
    assert!(names.iter().any(|name| name == ".."));
}

#[test]
fn unlink_removes_file() {
    let Some((mut inner, _guard)) = setup() else {
        return;
    };
    inner.delete(ROOT_INODE, FILE_NAME).unwrap();
    assert!(inner.lookup(ROOT_INODE, FILE_NAME).is_err());
}

#[test]
fn rmdir_removes_empty_directory() {
    let Some((mut inner, _guard)) = setup() else {
        return;
    };
    inner.delete(ROOT_INODE, DIR_NAME).unwrap();
    assert!(inner.lookup(ROOT_INODE, DIR_NAME).is_err());
}

/// Regression for a directory-corruption bug: deleting an *empty* file (whose
/// data cluster is 0) used to zero the reserved FAT[0] entry, after which a
/// subsequent write could be allocated cluster 0 — which maps to the root
/// directory's sector — overwriting directory entries with file content (the
/// user saw a garbage entry named after the file's bytes). Deleting an empty
/// file and then writing another file must keep the directory intact and the
/// content readable.
#[test]
fn delete_empty_file_then_write_keeps_directory_intact() {
    let Some((mut inner, _guard)) = setup() else {
        return;
    };

    // Create then delete an empty file (cluster 0), as `touch x && rm x` does.
    inner.create(ROOT_INODE, "empty.txt").unwrap();
    inner.delete(ROOT_INODE, "empty.txt").unwrap();

    // Now create another file and write content into it.
    let payload = b"hello\n";
    let ino = inner.create(ROOT_INODE, "data.txt").unwrap().ino.0;
    inner.write(ino, 0, payload).unwrap();

    // Content must round-trip exactly.
    let data = inner.read(ino, 0, payload.len() as u32).unwrap();
    assert_eq!(data, payload);

    // The directory listing must not contain any corrupted / garbage entries
    // (file content leaking into a directory slot shows up as control chars).
    let entries = inner.readdir(ROOT_INODE).unwrap();
    let names: Vec<&str> = entries.iter().map(|(_, _, name)| name.as_str()).collect();
    assert!(names.contains(&"data.txt"));
    assert!(!names.contains(&"empty.txt"));
    for name in &names {
        assert!(
            name.chars().all(|c| !c.is_control()),
            "directory contains a corrupted entry name: {name:?}"
        );
    }
    // The fixtures created by `populate` must still be present and intact.
    assert!(names.contains(&FILE_NAME));
    let hello_ino = inner.lookup(ROOT_INODE, FILE_NAME).unwrap().ino.0;
    let hello = inner.read(hello_ino, 0, 64).unwrap();
    assert_eq!(hello, FILE_CONTENT);
}

#[test]
fn rename_moves_file_preserving_content() {
    let Some((mut inner, _guard)) = setup() else {
        return;
    };
    inner
        .rename(ROOT_INODE, FILE_NAME, ROOT_INODE, "renamed.txt")
        .unwrap();

    assert!(inner.lookup(ROOT_INODE, FILE_NAME).is_err());
    let attr = inner.lookup(ROOT_INODE, "renamed.txt").unwrap();
    let data = inner
        .read(attr.ino.0, 0, FILE_CONTENT.len() as u32)
        .unwrap();
    assert_eq!(data, FILE_CONTENT);
}

#[test]
fn child_is_dir_distinguishes_types() {
    let Some((mut inner, _guard)) = setup() else {
        return;
    };
    assert!(!inner.child_is_dir(ROOT_INODE, FILE_NAME).unwrap());
    assert!(inner.child_is_dir(ROOT_INODE, DIR_NAME).unwrap());
    assert!(inner.child_is_dir(ROOT_INODE, "missing").is_err());
}

#[test]
fn created_file_has_nonepoch_timestamps() {
    let Some((mut inner, _guard)) = setup() else {
        return;
    };
    // A freshly created file gets its timestamps from the filesystem's time
    // manager (real wall-clock time under `std`), so they must not be the
    // hard-coded Unix epoch that the adapter previously reported.
    let attr = inner.create(ROOT_INODE, "stamped.txt").unwrap();
    assert!(
        attr.mtime > std::time::SystemTime::UNIX_EPOCH,
        "mtime should reflect the on-disk timestamp, not the Unix epoch"
    );
    assert!(
        attr.crtime > std::time::SystemTime::UNIX_EPOCH,
        "crtime should reflect the on-disk timestamp, not the Unix epoch"
    );

    // The same non-epoch timestamps survive a fresh lookup/getattr.
    let looked_up = inner.lookup(ROOT_INODE, "stamped.txt").unwrap();
    assert_eq!(looked_up.mtime, attr.mtime);
    assert_eq!(looked_up.crtime, attr.crtime);
}

#[test]
fn attributes_report_mounting_user() {
    let Some((mut inner, _guard)) = setup() else {
        return;
    };
    // FAT stores no ownership, so the adapter synthesises uid/gid from the
    // process that mounted the filesystem rather than reporting root.
    let attr = inner.getattr(ROOT_INODE).unwrap();
    assert_eq!(attr.uid, unsafe { libc::getuid() });
    assert_eq!(attr.gid, unsafe { libc::getgid() });
}

/// Hermetic tests for the sequential-write handle cache.
///
/// Unlike the tests above, these do not need `mkfs.fat`: they format a FAT32
/// image entirely in memory with the `fatfs` crate and drive [`Inner`] over an
/// in-RAM block device. This lets them run anywhere and directly assert that
/// contiguous writes reuse the cached open file (keeping its warm cluster-chain
/// writer alive) while other operations evict it.
mod hermetic {
    use std::io::Cursor;
    use std::sync::{Arc, Mutex};

    use vfat_rs::{BlockDevice, SectorId, VfatFS};

    use crate::{Inner, ROOT_INODE};

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
            let start = sector.0 as usize * 512 + offset;
            let n = buf.len().min(data.len().saturating_sub(start));
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
            let start = sector.0 as usize * 512 + offset;
            if start + buf.len() > data.len() {
                data.resize(start + buf.len(), 0);
            }
            data[start..start + buf.len()].copy_from_slice(buf);
            Ok(buf.len())
        }
    }

    /// Format a fresh 64 MiB FAT32 image in memory and wrap it in an [`Inner`].
    fn hermetic_inner() -> Inner {
        let mut image = vec![0u8; 64 * 1024 * 1024];
        {
            let cursor = Cursor::new(&mut image[..]);
            let options = fatfs::FormatVolumeOptions::new()
                .fat_type(fatfs::FatType::Fat32)
                .volume_label(*b"VFATFUSE   ");
            fatfs::format_volume(cursor, options).expect("format FAT32 image");
        }
        let device = MemoryBlockDevice(Arc::new(Mutex::new(image)));
        let fs = VfatFS::new(device, 0).expect("open VfatFS");
        Inner::new(fs)
    }

    /// Read the whole file `ino` back through the adapter.
    fn read_all(inner: &mut Inner, ino: u64, size: usize) -> Vec<u8> {
        let mut out = Vec::with_capacity(size);
        while out.len() < size {
            let chunk = inner
                .read(ino, out.len() as u64, (size - out.len()) as u32)
                .unwrap();
            assert!(!chunk.is_empty(), "unexpected EOF at {}/{size}", out.len());
            out.extend_from_slice(&chunk);
        }
        out
    }

    #[test]
    fn contiguous_writes_reuse_open_handle_and_roundtrip() {
        let mut inner = hermetic_inner();
        let attr = inner.create(ROOT_INODE, "seq.bin").unwrap();
        let ino = attr.ino.0;

        let chunk_size = 4096;
        let chunks = 512; // 2 MiB total
        let mut expected = Vec::new();
        for i in 0..chunks {
            let chunk: Vec<u8> = (0..chunk_size).map(|b| ((i + b) % 251) as u8).collect();
            let off = (i * chunk_size) as u64;
            let n = inner.write(ino, off, &chunk).unwrap();
            assert_eq!(n as usize, chunk_size);
            expected.extend_from_slice(&chunk);

            // Every contiguous write must keep the same handle warm.
            let open = inner
                .open_write
                .as_ref()
                .expect("handle should stay open across contiguous writes");
            assert_eq!(open.ino, ino);
            assert_eq!(open.next_offset, off + chunk_size as u64);
        }

        assert_eq!(read_all(&mut inner, ino, expected.len()), expected);
    }

    #[test]
    fn non_contiguous_write_rebuilds_handle() {
        let mut inner = hermetic_inner();
        let ino = inner.create(ROOT_INODE, "rand.bin").unwrap().ino.0;

        // Lay down 8 KiB sequentially (caches a handle at offset 8192).
        let a = vec![0xAAu8; 8192];
        inner.write(ino, 0, &a).unwrap();
        assert_eq!(inner.open_write.as_ref().unwrap().next_offset, 8192);

        // A write that does not continue the run must evict and rebuild.
        let b = vec![0xBBu8; 4096];
        inner.write(ino, 1024, &b).unwrap();
        assert_eq!(
            inner.open_write.as_ref().unwrap().next_offset,
            1024 + 4096,
            "handle should be rebuilt at the new write position"
        );

        let mut expected = a;
        expected[1024..1024 + 4096].copy_from_slice(&b);
        assert_eq!(read_all(&mut inner, ino, expected.len()), expected);
    }

    #[test]
    fn flush_and_truncate_evict_open_handle() {
        let mut inner = hermetic_inner();
        let ino = inner.create(ROOT_INODE, "evict.bin").unwrap().ino.0;

        inner.write(ino, 0, &[1u8; 4096]).unwrap();
        assert!(inner.open_write.is_some());
        inner.evict_open_write();
        assert!(inner.open_write.is_none());

        // Re-open via a fresh write, then a truncate must evict again.
        inner.write(ino, 4096, &[2u8; 4096]).unwrap();
        assert!(inner.open_write.is_some());
        inner.set_size(ino, 100).unwrap();
        assert!(
            inner.open_write.is_none(),
            "truncate/setattr must evict the cached writer"
        );
        assert_eq!(inner.getattr(ino).unwrap().size, 100);
    }

    #[test]
    fn switching_files_evicts_previous_handle() {
        let mut inner = hermetic_inner();
        let a = inner.create(ROOT_INODE, "a.bin").unwrap().ino.0;
        let b = inner.create(ROOT_INODE, "b.bin").unwrap().ino.0;

        inner.write(a, 0, &[0xA1u8; 2048]).unwrap();
        assert_eq!(inner.open_write.as_ref().unwrap().ino, a);

        // Writing a different file evicts a's handle and caches b's.
        inner.write(b, 0, &[0xB1u8; 2048]).unwrap();
        assert_eq!(inner.open_write.as_ref().unwrap().ino, b);

        assert_eq!(read_all(&mut inner, a, 2048), vec![0xA1u8; 2048]);
        assert_eq!(read_all(&mut inner, b, 2048), vec![0xB1u8; 2048]);
    }

    #[test]
    fn contiguous_reads_reuse_open_read_handle() {
        let mut inner = hermetic_inner();
        let ino = inner.create(ROOT_INODE, "r.bin").unwrap().ino.0;

        let total = 256 * 1024;
        let data: Vec<u8> = (0..total).map(|i| (i % 251) as u8).collect();
        inner.write(ino, 0, &data).unwrap();
        // A write must not leave a read handle cached.
        assert!(inner.open_read.is_none());

        // Read back in 4 KiB chunks; each contiguous read keeps the handle warm.
        let chunk = 4096u32;
        let mut got = Vec::new();
        while got.len() < total {
            let off = got.len() as u64;
            let part = inner.read(ino, off, chunk).unwrap();
            assert!(!part.is_empty());
            got.extend_from_slice(&part);
            let open = inner
                .open_read
                .as_ref()
                .expect("read handle should stay open across contiguous reads");
            assert_eq!(open.ino, ino);
            assert_eq!(open.next_offset, off + part.len() as u64);
        }
        assert_eq!(got, data);
    }

    #[test]
    fn write_evicts_cached_read_handle() {
        let mut inner = hermetic_inner();
        let ino = inner.create(ROOT_INODE, "rw.bin").unwrap().ino.0;
        inner.write(ino, 0, &[7u8; 8192]).unwrap();

        // Prime the read cache.
        let _ = inner.read(ino, 0, 4096).unwrap();
        assert!(inner.open_read.is_some());

        // A subsequent write must drop the stale read handle.
        inner.write(ino, 8192, &[9u8; 4096]).unwrap();
        assert!(
            inner.open_read.is_none(),
            "a write must evict the cached read handle"
        );

        let mut expected = vec![7u8; 8192];
        expected.extend_from_slice(&[9u8; 4096]);
        assert_eq!(read_all(&mut inner, ino, expected.len()), expected);
    }

    #[test]
    fn statfs_reports_free_space_that_shrinks_on_write() {
        let mut inner = hermetic_inner();
        let (blocks, bfree0, bsize) = inner.statfs().unwrap();
        assert!(blocks > 0 && bsize > 0);
        assert!(bfree0 > 0 && bfree0 <= blocks);

        // Writing 1 MiB must reduce the free-block count by exactly 1 MiB worth.
        let ino = inner.create(ROOT_INODE, "fill.bin").unwrap().ino.0;
        let one_mib = 1024 * 1024;
        inner.write(ino, 0, &vec![1u8; one_mib]).unwrap();
        inner.evict_open_handles();

        let (_, bfree1, _) = inner.statfs().unwrap();
        let used_blocks = bfree0 - bfree1;
        assert_eq!(used_blocks, one_mib as u64 / bsize as u64);
    }

    #[test]
    fn release_evicts_open_handles() {
        let mut inner = hermetic_inner();
        let ino = inner.create(ROOT_INODE, "rel.bin").unwrap().ino.0;
        inner.write(ino, 0, &[1u8; 4096]).unwrap();
        assert!(inner.open_write.is_some());
        let _ = inner.read(ino, 0, 1024); // primes nothing (write path evicts read), but exercise
        inner.evict_open_handles();
        assert!(inner.open_write.is_none() && inner.open_read.is_none());
    }

    #[test]
    fn set_times_persists_mtime_and_crtime() {
        use std::time::{Duration, SystemTime};

        let mut inner = hermetic_inner();
        let ino = inner.create(ROOT_INODE, "dated.txt").unwrap().ino.0;

        // 2007-06-15 12:30:20 UTC (crtime) and 2021-04-23 08:15:00 UTC (mtime),
        // both even seconds so they survive VFAT's 2-second resolution exactly.
        let crtime = SystemTime::UNIX_EPOCH + Duration::from_secs(1_181_910_620);
        let mtime = SystemTime::UNIX_EPOCH + Duration::from_secs(1_619_165_700);

        let attr = inner
            .set_times(
                ino,
                None,
                Some(fuser::TimeOrNow::SpecificTime(mtime)),
                Some(crtime),
            )
            .unwrap();
        assert_eq!(attr.mtime, mtime, "returned attr mtime");
        assert_eq!(attr.crtime, crtime, "returned attr crtime");

        // Re-read from disk via getattr to confirm the entry was persisted.
        let reread = inner.getattr(ino).unwrap();
        assert_eq!(reread.mtime, mtime, "persisted mtime");
        assert_eq!(reread.crtime, crtime, "persisted crtime");
    }

    #[test]
    fn set_times_now_updates_to_recent_time() {
        use std::time::{Duration, SystemTime};

        let mut inner = hermetic_inner();
        let ino = inner.create(ROOT_INODE, "touched.txt").unwrap().ino.0;

        let before = SystemTime::now() - Duration::from_secs(4);
        let attr = inner
            .set_times(ino, None, Some(fuser::TimeOrNow::Now), None)
            .unwrap();

        // `Now` must resolve to a real recent time, not the 1970 epoch.
        assert!(
            attr.mtime >= before,
            "mtime {:?} should be at/after {:?}",
            attr.mtime,
            before
        );
        assert!(attr.mtime > SystemTime::UNIX_EPOCH + Duration::from_secs(1_000_000_000));
    }

    #[test]
    fn set_times_atime_only_does_not_change_mtime() {
        use std::time::{Duration, SystemTime};

        let mut inner = hermetic_inner();
        let ino = inner.create(ROOT_INODE, "atime.txt").unwrap().ino.0;

        // Pin a known mtime first.
        let mtime = SystemTime::UNIX_EPOCH + Duration::from_secs(1_619_165_700);
        inner
            .set_times(ino, None, Some(fuser::TimeOrNow::SpecificTime(mtime)), None)
            .unwrap();

        // An atime-only request (e.g. `touch -a`) must leave mtime untouched: FAT
        // has no last-access time, so this is a no-op for the on-disk entry.
        let far_future = SystemTime::UNIX_EPOCH + Duration::from_secs(4_000_000_000);
        let attr = inner
            .set_times(
                ino,
                Some(fuser::TimeOrNow::SpecificTime(far_future)),
                None,
                None,
            )
            .unwrap();
        assert_eq!(
            attr.mtime, mtime,
            "atime-only setattr must not change mtime"
        );

        let reread = inner.getattr(ino).unwrap();
        assert_eq!(reread.mtime, mtime, "persisted mtime must be unchanged");
    }
}
