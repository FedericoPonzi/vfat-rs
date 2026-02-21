//! Traits that the user must implement.
//!
//! The OS should provide:
//! * An implementation for the `TimeManagerTrait`. This is used for timestamping file creation and update.
//! * An implementation for the `BlockDevice` trait. This is used to interact with the disk.
//! * Alloc support. It is mostly used to allocate some vec and strings, it might be put behind a feature flag later on.
//!
use crate::api::timestamp::VfatTimestamp;
use crate::{error, SectorId};
use core::fmt::Debug;

/// An interface to the OS-owned timer. Needed for timestamping file creations and update.
pub trait TimeManagerTrait: Debug + Send + Sync {
    /// Get the current Unix timestamp in milliseconds.
    /// The number of seconds since January 1, 1970, 00:00:00 UTC
    /// TODO negative dates? Should this be i64?
    fn get_current_timestamp(&self) -> u64;
    /// Convert the current Unix timestamp into a [`VfatTimestamp`].
    fn get_current_vfat_timestamp(&self) -> VfatTimestamp {
        VfatTimestamp::from(self.get_current_timestamp())
    }
}

/// A block device is a computer data storage device that supports reading
/// and (optionally) writing data in fixed-size blocks, sectors, or clusters.
/// These blocks are generally 512 bytes or a multiple thereof in size.
/// TODO: move _offset functions to cachedpartition only.
pub trait BlockDevice {
    /// Sector size in bytes.
    fn sector_size(&self) -> usize {
        512
    }

    /// Read sector `n` in `buf`, up to min(self.sector_size() and buf.size()).
    /// Returns the amount of the bytes read.
    ///
    /// Needs to be mutable because, for instance we might
    /// need to use seek to move the pointer on the file
    fn read_sector(&mut self, sector: SectorId, buf: &mut [u8]) -> error::Result<usize> {
        self.read_sector_offset(sector, 0, buf) //TODO: this is wrong. it should keep track of offset somewhere.
    }

    /// Read a sector starting from an offset.
    fn read_sector_offset(
        &mut self,
        sector: SectorId,
        offset: usize,
        buf: &mut [u8],
    ) -> error::Result<usize>;

    /// Write an entire sector.
    fn write_sector(&mut self, sector: SectorId, buf: &[u8]) -> error::Result<usize> {
        self.write_sector_offset(sector, 0, buf)
    }

    /// write start from an offset in a sector
    fn write_sector_offset(
        &mut self,
        sector: SectorId,
        offset: usize,
        buf: &[u8],
    ) -> error::Result<usize>;

    /// A human readable name for this device
    fn get_canonical_name() -> &'static str
    where
        Self: Sized,
    {
        "Block Device"
    }
}
