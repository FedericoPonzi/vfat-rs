//! vfat-rs is a simple vfat (fat32) implementation in Rust.
//! Use it in your custom kernel or integrate it in your user level application.
#![doc = include_str!("../README.md")]
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![deny(missing_docs)]
//#![deny(unsafe_code)]
// to remove:
//#![allow(unused_variables)]
//#![allow(dead_code)]
#![allow(clippy::too_many_arguments)]

extern crate alloc;
extern crate core;

use alloc::sync::Arc;

use api::raw_directory_entry::{
    Attributes, RegularDirectoryEntry, UnknownDirectoryEntry, VfatDirectoryEntry,
};
pub use api::EntryType;
pub use api::{Directory, DirectoryEntry, Metadata, VfatMetadataTrait};
pub(crate) use cache::CachedPartition;
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
/// VfatRs error definitions
mod error;
mod fat_table;
#[cfg(feature = "std")]
mod fileblockdevice;
mod formats;
/// I/O traits and error types.
pub mod io;
mod macros;
/// Master Boot Record parsing.
pub mod mbr;
mod time;
/// OS-integration traits (`BlockDevice`, `TimeManagerTrait`).
pub mod traits;
mod vfat;

const EBPF_VFAT_MAGIC: u8 = 0x28;
const EBPF_VFAT_MAGIC_ALT: u8 = 0x29;

/// Why Arc? Because CachedPartition owns the block device. And
/// Vfat needs to be cloned, and potentially we could send references across threads.
type ArcMutex<CachedPartition> = Arc<CachedPartition>;

#[cfg(feature = "std")]
pub use time::TimeManagerChronos;
pub use time::TimeManagerNoop;

#[cfg(feature = "std")]
pub use fileblockdevice::FilebackedBlockDevice;

pub use traits::{BlockDevice, TimeManagerTrait};
