use std::fs::{self, OpenOptions};
use std::path::PathBuf;

use super::file_blockdev::FilebackedBlockDevice;
use super::setup::{self, VfatFsRandomPath};
use vfat_rs::mbr::MasterBootRecord;
use vfat_rs::{BlockDevice, SectorId, VfatFS};

/// A reusable filesystem image that avoids repeated mount/unmount.
///
/// Call `setup()` once (which runs setup.sh with mount/umount), then use
/// `open_vfat()` / `open_vfat_cached()` to create independent VfatFS
/// instances from cheap file copies — no further mount/umount needed.
pub struct BenchFs {
    _setup: VfatFsRandomPath,
    source_path: PathBuf,
    copy_counter: std::sync::atomic::AtomicU32,
    copies: std::sync::Mutex<Vec<PathBuf>>,
}

impl BenchFs {
    pub fn new() -> Self {
        let setup = setup::setup();
        let source_path = setup.fs_path.clone();
        Self {
            _setup: setup,
            source_path,
            copy_counter: std::sync::atomic::AtomicU32::new(0),
            copies: std::sync::Mutex::new(Vec::new()),
        }
    }

    fn copy_image(&self) -> PathBuf {
        let n = self
            .copy_counter
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let copy_path = self.source_path.with_extension(format!("bench.{}", n));
        fs::copy(&self.source_path, &copy_path).expect("Failed to copy FS image");
        self.copies.lock().unwrap().push(copy_path.clone());
        copy_path
    }

    fn open_image(path: &PathBuf) -> (FilebackedBlockDevice, MasterBootRecord) {
        let mut dev = FilebackedBlockDevice {
            image: OpenOptions::new()
                .read(true)
                .write(true)
                .open(path)
                .unwrap(),
        };
        let mut buf = [0; 512];
        dev.read_sector(SectorId(0), &mut buf).unwrap();
        let mbr = MasterBootRecord::from(buf);
        (dev, mbr)
    }

    pub fn open_vfat(&self) -> VfatFS {
        let path = self.copy_image();
        let (dev, mbr) = Self::open_image(&path);
        VfatFS::new(dev, mbr.partitions[0].start_sector).unwrap()
    }

    pub fn open_vfat_cached(&self, cache_capacity: usize) -> VfatFS {
        let path = self.copy_image();
        let (dev, mbr) = Self::open_image(&path);
        VfatFS::new_with_cache(
            dev,
            mbr.partitions[0].start_sector,
            vfat_rs::TimeManagerNoop::new(),
            cache_capacity,
        )
        .unwrap()
    }
}

impl Drop for BenchFs {
    fn drop(&mut self) {
        for path in self.copies.lock().unwrap().iter() {
            let _ = fs::remove_file(path);
        }
    }
}
