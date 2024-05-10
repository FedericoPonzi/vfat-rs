//! ntfs-rs is a simple ntfs implementation in Rust.
#![cfg_attr(not(any(test, feature = "std")), no_std)]
//#![deny(missing_docs)]
//#![deny(unsafe_code)]
// to remove:
//#![allow(unused_variables)]
//#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]

extern crate alloc;
extern crate core;

use alloc::sync::Arc;

use api::directory_entry::{
    Attributes, RegularDirectoryEntry, UnknownDirectoryEntry, VfatDirectoryEntry,
};
pub use api::EntryType;
pub use api::{Directory, Metadata, VfatEntry, VfatMetadataTrait};
pub(crate) use cache::CachedPartition;
pub use device::BlockDevice;
#[cfg(feature = "std")]
pub use device::FilebackedBlockDevice;
pub use error::{Result, VfatRsError};
pub(crate) use formats::cluster_id::ClusterId;
#[cfg(not(feature = "std"))]
pub use formats::path::PathBuf;
#[cfg(feature = "std")]
pub use std::path::PathBuf;

pub use formats::sector_id::SectorId;
pub use vfat::VfatFS;

mod api;
mod cache;
mod cluster;
mod device;
/// NtfsRs error definitions
mod error;
mod fat_table;
mod formats;
pub mod io;
mod macros;
/// A simple Master Booot Record implementation
pub mod mbr;
mod vfat;

const EBPF_VFAT_MAGIC: u8 = 0x28;
const EBPF_VFAT_MAGIC_ALT: u8 = 0x29;

/// Why Arc? Because CachedPartition owns the block device. And
/// Vfat needs to be cloned, and potentially we could send references across threads.
type ArcMutex<CachedPartition> = Arc<CachedPartition>;

pub use traits::{TimeManagerNoop, TimeManagerTrait};
pub mod traits {
    use crate::api::timestamp::VfatTimestamp;
    use alloc::sync::Arc;
    use core::fmt::Debug;

    // An interface to the OS-owned timer. Needed for timestamping file creations and update.
    pub trait TimeManagerTrait: Debug {
        /// Get the current Unix timestamp in milliseconds.
        /// The number of seconds since January 1, 1970, 00:00:00 UTC
        /// TODO negative dates? Should this be i64?
        fn get_current_timestamp(&self) -> u64;
        fn get_current_vfat_timestamp(&self) -> VfatTimestamp {
            VfatTimestamp::from(self.get_current_timestamp())
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct TimeManagerNoop {}
    impl TimeManagerNoop {
        pub fn new() -> Self {
            Default::default()
        }
        pub fn new_arc() -> Arc<Self> {
            Arc::new(Self {})
        }
    }
    impl TimeManagerTrait for TimeManagerNoop {
        fn get_current_timestamp(&self) -> u64 {
            0
        }
    }

    #[cfg(feature = "std")]
    #[derive(Clone, Debug)]
    pub struct TimeManagerChronos {}
    #[cfg(feature = "std")]
    impl TimeManagerChronos {
        pub(crate) fn new() -> Self {
            Self {}
        }
    }
    #[cfg(feature = "std")]
    impl TimeManagerTrait for TimeManagerChronos {
        fn get_current_timestamp(&self) -> u64 {
            use chrono::Utc;
            let now = Utc::now();
            let seconds_since_epoch: i64 = now.timestamp();
            // I guess it's an i64 because of underflow for dates before 1970
            seconds_since_epoch as u64
        }
    }
}
