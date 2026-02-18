use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;

use log::info;
use spin::mutex::SpinMutex;

use crate::error::Result;
use crate::formats::cluster_id::ClusterId;
use crate::traits::BlockDevice;
use crate::SectorId;

/// A cached sector entry.
struct CacheEntry {
    sector: SectorId,
    data: Vec<u8>,
    dirty: bool,
    /// Monotonic counter for LRU eviction.
    last_used: u64,
}

/// An interface to the underlying Block Device with an optional write-back sector cache.
///
/// When `cache_capacity` is 0, all reads and writes pass directly through to the
/// device (no caching overhead). When non-zero, sectors are cached in memory and
/// dirty sectors are flushed to the device on [`flush()`](Self::flush), eviction,
/// or [`Drop`].
pub(crate) struct CachedPartition {
    device: SpinMutex<Box<dyn BlockDevice + Send>>,
    pub(crate) sector_size: usize,
    pub(crate) fat_start_sector: SectorId,
    /// How many sectors are mapped to a single cluster
    pub(crate) sectors_per_cluster: u32,
    /// First sector containing actual data - after all FAT tables.
    pub(crate) data_start_sector: SectorId,
    /// Number of FAT copies (typically 2 for redundancy)
    pub(crate) fat_amount: u8,
    /// Number of sectors per FAT table
    pub(crate) sectors_per_fat: u32,
    /// Sector cache. Protected by its own lock to avoid holding the device lock.
    cache: SpinMutex<SectorCache>,
}

struct SectorCache {
    entries: Vec<CacheEntry>,
    capacity: usize,
    /// Monotonically increasing counter for LRU ordering.
    access_counter: u64,
}

impl SectorCache {
    fn new(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
            capacity,
            access_counter: 0,
        }
    }

    fn next_tick(&mut self) -> u64 {
        self.access_counter += 1;
        self.access_counter
    }

    /// Find a cached entry by sector id, returning its index.
    fn find(&self, sector: SectorId) -> Option<usize> {
        self.entries.iter().position(|e| e.sector == sector)
    }

    /// Find the index of the least recently used entry.
    fn lru_index(&self) -> Option<usize> {
        self.entries
            .iter()
            .enumerate()
            .min_by_key(|(_, e)| e.last_used)
            .map(|(i, _)| i)
    }
}

impl CachedPartition {
    /// Create a new CachedPartition with caching disabled (capacity 0).
    #[cfg(test)]
    pub fn new<T>(
        device: T,
        sector_size: usize,
        fat_start_sector: SectorId,
        sectors_per_cluster: u32,
        data_start_sector: SectorId,
        fat_amount: u8,
        sectors_per_fat: u32,
    ) -> Self
    where
        T: BlockDevice + Send + 'static,
    {
        Self::new_with_cache(
            device,
            sector_size,
            fat_start_sector,
            sectors_per_cluster,
            data_start_sector,
            fat_amount,
            sectors_per_fat,
            0, // default: no caching (preserves existing behavior)
        )
    }

    pub fn new_with_cache<T>(
        device: T,
        sector_size: usize,
        fat_start_sector: SectorId,
        sectors_per_cluster: u32,
        data_start_sector: SectorId,
        fat_amount: u8,
        sectors_per_fat: u32,
        cache_capacity: usize,
    ) -> Self
    where
        T: BlockDevice + Send + 'static,
    {
        info!(
            "Creating cached partition (cache_capacity={})",
            cache_capacity
        );
        Self {
            device: SpinMutex::new(Box::new(device)),
            sector_size,
            fat_start_sector,
            sectors_per_cluster,
            data_start_sector,
            fat_amount,
            sectors_per_fat,
            cache: SpinMutex::new(SectorCache::new(cache_capacity)),
        }
    }

    /// Flush all dirty cached sectors to the device.
    pub fn flush(&self) -> Result<()> {
        let mut cache = self.cache.lock();
        let mut device = self.device.lock();
        for entry in cache.entries.iter_mut() {
            if entry.dirty {
                device.write_sector(entry.sector, &entry.data)?;
                entry.dirty = false;
            }
        }
        Ok(())
    }

    /// Flush a single cache entry to the device (caller must hold both locks).
    fn flush_entry(device: &mut Box<dyn BlockDevice + Send>, entry: &mut CacheEntry) -> Result<()> {
        if entry.dirty {
            device.write_sector(entry.sector, &entry.data)?;
            entry.dirty = false;
        }
        Ok(())
    }

    pub(crate) fn read_sector(&self, sector: SectorId, buf: &mut [u8]) -> Result<usize> {
        let mut cache = self.cache.lock();
        if cache.capacity == 0 {
            drop(cache);
            let mut dev_lock = self.device.lock();
            return dev_lock.read_sector(sector, buf);
        }

        // Cache hit
        if let Some(idx) = cache.find(sector) {
            let tick = cache.next_tick();
            let entry = &mut cache.entries[idx];
            entry.last_used = tick;
            let len = core::cmp::min(buf.len(), entry.data.len());
            buf[..len].copy_from_slice(&entry.data[..len]);
            return Ok(len);
        }

        // Cache miss: read from device
        let mut dev_lock = self.device.lock();
        let mut sector_buf = vec![0u8; self.sector_size];
        let read = dev_lock.read_sector(sector, &mut sector_buf)?;
        let len = core::cmp::min(buf.len(), read);
        buf[..len].copy_from_slice(&sector_buf[..len]);

        // Insert into cache (evict LRU if full)
        if cache.entries.len() >= cache.capacity {
            if let Some(lru) = cache.lru_index() {
                Self::flush_entry(&mut dev_lock, &mut cache.entries[lru])?;
                let tick = cache.next_tick();
                cache.entries[lru] = CacheEntry {
                    sector,
                    data: sector_buf,
                    dirty: false,
                    last_used: tick,
                };
            }
        } else {
            let tick = cache.next_tick();
            cache.entries.push(CacheEntry {
                sector,
                data: sector_buf,
                dirty: false,
                last_used: tick,
            });
        }

        Ok(len)
    }

    pub(crate) fn read_sector_offset(
        self: Arc<Self>,
        sector: SectorId,
        offset: usize,
        buf: &mut [u8],
    ) -> Result<usize> {
        let mut cache = self.cache.lock();
        if cache.capacity == 0 {
            drop(cache);
            let mut dev_lock = self.device.lock();
            return dev_lock.read_sector_offset(sector, offset, buf);
        }

        // Cache hit
        if let Some(idx) = cache.find(sector) {
            let tick = cache.next_tick();
            let entry = &mut cache.entries[idx];
            entry.last_used = tick;
            let available = entry.data.len().saturating_sub(offset);
            let len = core::cmp::min(buf.len(), available);
            buf[..len].copy_from_slice(&entry.data[offset..offset + len]);
            return Ok(len);
        }

        // Cache miss: read full sector from device, cache it, return slice
        let mut dev_lock = self.device.lock();
        let mut sector_buf = vec![0u8; self.sector_size];
        dev_lock.read_sector(sector, &mut sector_buf)?;
        let available = sector_buf.len().saturating_sub(offset);
        let len = core::cmp::min(buf.len(), available);
        buf[..len].copy_from_slice(&sector_buf[offset..offset + len]);

        if cache.entries.len() >= cache.capacity {
            if let Some(lru) = cache.lru_index() {
                Self::flush_entry(&mut dev_lock, &mut cache.entries[lru])?;
                let tick = cache.next_tick();
                cache.entries[lru] = CacheEntry {
                    sector,
                    data: sector_buf,
                    dirty: false,
                    last_used: tick,
                };
            }
        } else {
            let tick = cache.next_tick();
            cache.entries.push(CacheEntry {
                sector,
                data: sector_buf,
                dirty: false,
                last_used: tick,
            });
        }

        Ok(len)
    }

    #[allow(unused)]
    fn write_sector(self: Arc<Self>, sector: SectorId, buf: &[u8]) -> Result<usize> {
        let mut cache = self.cache.lock();
        if cache.capacity == 0 {
            drop(cache);
            let mut dev_lock = self.device.lock();
            return dev_lock.write_sector(sector, buf);
        }

        let len = buf.len();
        let tick = cache.next_tick();

        // Cache hit: update in place
        if let Some(idx) = cache.find(sector) {
            let entry = &mut cache.entries[idx];
            entry.data[..len].copy_from_slice(buf);
            entry.dirty = true;
            entry.last_used = tick;
            return Ok(len);
        }

        // Cache miss: insert new dirty entry
        let mut data = vec![0u8; self.sector_size];
        data[..len].copy_from_slice(buf);

        if cache.entries.len() >= cache.capacity {
            if let Some(lru) = cache.lru_index() {
                let mut dev_lock = self.device.lock();
                Self::flush_entry(&mut dev_lock, &mut cache.entries[lru])?;
                cache.entries[lru] = CacheEntry {
                    sector,
                    data,
                    dirty: true,
                    last_used: tick,
                };
            }
        } else {
            cache.entries.push(CacheEntry {
                sector,
                data,
                dirty: true,
                last_used: tick,
            });
        }

        Ok(len)
    }

    pub(crate) fn write_sector_offset(
        self: Arc<Self>,
        sector: SectorId,
        offset: usize,
        buf: &[u8],
    ) -> Result<usize> {
        let mut cache = self.cache.lock();
        if cache.capacity == 0 {
            drop(cache);
            let mut dev_lock = self.device.lock();
            return dev_lock.write_sector_offset(sector, offset, buf);
        }

        let len = buf.len();
        let tick = cache.next_tick();

        // Cache hit: update in place
        if let Some(idx) = cache.find(sector) {
            let entry = &mut cache.entries[idx];
            entry.data[offset..offset + len].copy_from_slice(buf);
            entry.dirty = true;
            entry.last_used = tick;
            return Ok(len);
        }

        // Cache miss: read full sector first, then apply the write
        let mut dev_lock = self.device.lock();
        let mut data = vec![0u8; self.sector_size];
        dev_lock.read_sector(sector, &mut data)?;
        data[offset..offset + len].copy_from_slice(buf);

        if cache.entries.len() >= cache.capacity {
            if let Some(lru) = cache.lru_index() {
                Self::flush_entry(&mut dev_lock, &mut cache.entries[lru])?;
                cache.entries[lru] = CacheEntry {
                    sector,
                    data,
                    dirty: true,
                    last_used: tick,
                };
            }
        } else {
            cache.entries.push(CacheEntry {
                sector,
                data,
                dirty: true,
                last_used: tick,
            });
        }

        Ok(len)
    }

    /// Converts a cluster (a FAT concept) to a sector (a BlockDevice concept).
    ///
    /// To do so, it uses some useful info from the BPB section.
    pub(crate) fn cluster_to_sector(&self, cluster: ClusterId) -> SectorId {
        let selected_sector = u32::from(cluster).saturating_sub(2) * self.sectors_per_cluster;
        let sect = self.data_start_sector.0 + selected_sector;
        SectorId(sect)
    }

    #[allow(unused)]
    fn get_canonical_name() -> &'static str
    where
        Self: Sized,
    {
        "CachePartition"
    }
}

impl Drop for CachedPartition {
    fn drop(&mut self) {
        // Best-effort flush on drop. We can't propagate errors here,
        // so dirty data is silently lost if the device write fails.
        let cache = self.cache.get_mut();
        let device = self.device.get_mut();
        for entry in cache.entries.iter_mut() {
            if entry.dirty {
                let _ = device.write_sector(entry.sector, &entry.data);
                entry.dirty = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::sync::Arc;
    use spin::mutex::SpinMutex;

    /// A simple in-memory block device that tracks reads and writes.
    struct MemBlockDevice {
        /// Flat storage: sector N occupies bytes [N*512 .. (N+1)*512].
        data: Arc<SpinMutex<Vec<u8>>>,
        read_count: Arc<SpinMutex<u32>>,
        write_count: Arc<SpinMutex<u32>>,
    }

    impl MemBlockDevice {
        fn new(num_sectors: usize) -> Self {
            Self {
                data: Arc::new(SpinMutex::new(vec![0u8; num_sectors * 512])),
                read_count: Arc::new(SpinMutex::new(0)),
                write_count: Arc::new(SpinMutex::new(0)),
            }
        }

        fn reads(&self) -> u32 {
            *self.read_count.lock()
        }

        fn writes(&self) -> u32 {
            *self.write_count.lock()
        }

        /// Read a sector directly from backing store (bypasses cache).
        fn read_raw(&self, sector: SectorId, buf: &mut [u8]) {
            let start = sector.0 as usize * 512;
            let len = core::cmp::min(buf.len(), 512);
            buf[..len].copy_from_slice(&self.data.lock()[start..start + len]);
        }
    }

    impl Clone for MemBlockDevice {
        fn clone(&self) -> Self {
            Self {
                data: self.data.clone(),
                read_count: self.read_count.clone(),
                write_count: self.write_count.clone(),
            }
        }
    }

    impl BlockDevice for MemBlockDevice {
        fn read_sector_offset(
            &mut self,
            sector: SectorId,
            offset: usize,
            buf: &mut [u8],
        ) -> Result<usize> {
            *self.read_count.lock() += 1;
            let start = sector.0 as usize * 512 + offset;
            let data = self.data.lock();
            let len = core::cmp::min(buf.len(), data.len() - start);
            buf[..len].copy_from_slice(&data[start..start + len]);
            Ok(len)
        }

        fn write_sector_offset(
            &mut self,
            sector: SectorId,
            offset: usize,
            buf: &[u8],
        ) -> Result<usize> {
            *self.write_count.lock() += 1;
            let start = sector.0 as usize * 512 + offset;
            let mut data = self.data.lock();
            data[start..start + buf.len()].copy_from_slice(buf);
            Ok(buf.len())
        }

        fn get_canonical_name() -> &'static str
        where
            Self: Sized,
        {
            "MemBlockDevice"
        }
    }

    fn make_cached(dev: MemBlockDevice, cache_capacity: usize) -> Arc<CachedPartition> {
        Arc::new(CachedPartition::new_with_cache(
            dev,
            512,
            SectorId(0),
            1,
            SectorId(10),
            1,
            1,
            cache_capacity,
        ))
    }

    #[test]
    fn test_cache_zero_capacity_passthrough() {
        let dev = MemBlockDevice::new(16);
        let dev_clone = dev.clone();
        let cp = make_cached(dev, 0);

        // Write goes directly to device
        cp.clone()
            .write_sector_offset(SectorId(1), 0, &[0xAA; 4])
            .unwrap();
        assert_eq!(dev_clone.writes(), 1);

        // Read goes directly to device
        let mut buf = [0u8; 4];
        cp.read_sector(SectorId(1), &mut buf).unwrap();
        assert_eq!(dev_clone.reads(), 1);
        assert_eq!(buf, [0xAA; 4]);
    }

    #[test]
    fn test_cache_read_hit_avoids_device() {
        let dev = MemBlockDevice::new(16);
        let dev_clone = dev.clone();
        let cp = make_cached(dev, 4);

        // First read: cache miss → device read
        let mut buf = [0u8; 512];
        cp.read_sector(SectorId(1), &mut buf).unwrap();
        assert_eq!(dev_clone.reads(), 1);

        // Second read: cache hit → no device read
        cp.read_sector(SectorId(1), &mut buf).unwrap();
        assert_eq!(dev_clone.reads(), 1);
    }

    #[test]
    fn test_cache_write_is_deferred() {
        let dev = MemBlockDevice::new(16);
        let dev_clone = dev.clone();
        let cp = make_cached(dev, 4);

        // Populate cache with a read first
        let mut buf = [0u8; 512];
        cp.read_sector(SectorId(1), &mut buf).unwrap();

        // Write to cached sector: should NOT hit device yet
        cp.clone()
            .write_sector_offset(SectorId(1), 0, &[0xBB; 4])
            .unwrap();
        assert_eq!(dev_clone.writes(), 0);

        // Flush: now device should be written
        cp.flush().unwrap();
        assert_eq!(dev_clone.writes(), 1);

        // Verify data on device
        let mut raw = [0u8; 4];
        dev_clone.read_raw(SectorId(1), &mut raw);
        assert_eq!(raw, [0xBB; 4]);
    }

    #[test]
    fn test_cache_lru_eviction_flushes_dirty() {
        let dev = MemBlockDevice::new(16);
        let dev_clone = dev.clone();
        // Cache of size 2
        let cp = make_cached(dev, 2);

        // Read and dirty sector 1 (last_used=2 after write)
        let mut buf = [0u8; 512];
        cp.read_sector(SectorId(1), &mut buf).unwrap();
        cp.clone()
            .write_sector_offset(SectorId(1), 0, &[0xCC; 4])
            .unwrap();
        assert_eq!(dev_clone.writes(), 0); // still cached

        // Read sector 2: sector 1 is LRU but cache not full yet
        cp.read_sector(SectorId(2), &mut buf).unwrap();

        // Read sector 3: evicts LRU (sector 1, which is dirty) → device write
        cp.read_sector(SectorId(3), &mut buf).unwrap();
        assert_eq!(dev_clone.writes(), 1); // sector 1 flushed on eviction

        // Verify evicted data landed on device
        let mut raw = [0u8; 4];
        dev_clone.read_raw(SectorId(1), &mut raw);
        assert_eq!(raw, [0xCC; 4]);
    }

    #[test]
    fn test_cache_drop_flushes_dirty() {
        let dev = MemBlockDevice::new(16);
        let dev_clone = dev.clone();

        {
            let cp = make_cached(dev, 4);
            // Read then dirty a sector
            let mut buf = [0u8; 512];
            cp.read_sector(SectorId(5), &mut buf).unwrap();
            cp.clone()
                .write_sector_offset(SectorId(5), 0, &[0xDD; 4])
                .unwrap();
            assert_eq!(dev_clone.writes(), 0);
            // cp dropped here
        }

        // After drop, dirty sector should be flushed
        assert_eq!(dev_clone.writes(), 1);
        let mut raw = [0u8; 4];
        dev_clone.read_raw(SectorId(5), &mut raw);
        assert_eq!(raw, [0xDD; 4]);
    }

    #[test]
    fn test_cache_flush_idempotent() {
        let dev = MemBlockDevice::new(16);
        let dev_clone = dev.clone();
        let cp = make_cached(dev, 4);

        let mut buf = [0u8; 512];
        cp.read_sector(SectorId(1), &mut buf).unwrap();
        cp.clone()
            .write_sector_offset(SectorId(1), 0, &[0xEE; 4])
            .unwrap();

        cp.flush().unwrap();
        assert_eq!(dev_clone.writes(), 1);

        // Second flush: nothing dirty → no additional writes
        cp.flush().unwrap();
        assert_eq!(dev_clone.writes(), 1);
    }
}
