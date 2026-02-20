use binrw::BinRead;

use crate::const_assert_size;

/// FSInfo sector signature constants per FAT32 specification.
const FSINFO_LEAD_SIG: u32 = 0x41615252;
const FSINFO_STRUC_SIG: u32 = 0x61417272;
const FSINFO_TRAIL_SIG: u32 = 0xAA550000;

/// Value indicating the free count / next free fields are unknown.
const FSINFO_UNKNOWN: u32 = 0xFFFFFFFF;

/// FAT32 FSInfo sector (sector indicated by `fsinfo_sector` in the EBPB).
///
/// Stores advisory hints about the filesystem:
/// - `free_count`: last known number of free clusters
/// - `nxt_free`: hint for where to start searching for free clusters
///
/// Both values are *hints* only — they may be stale after an unclean unmount.
/// The implementation must handle invalid/stale values gracefully.
#[derive(Debug, Copy, Clone, BinRead)]
#[repr(C, packed)]
pub(crate) struct FSInfoSector {
    lead_sig: u32,
    _reserved1: [u8; 480],
    struc_sig: u32,
    /// Last known free cluster count (0xFFFFFFFF = unknown).
    pub free_count: u32,
    /// Hint for the next free cluster to start searching from (0xFFFFFFFF = unknown).
    pub nxt_free: u32,
    _reserved2: [u8; 12],
    trail_sig: u32,
}

const_assert_size!(FSInfoSector, 512);

impl FSInfoSector {
    /// Returns true if both lead and struct signatures are valid.
    pub fn is_valid(&self) -> bool {
        self.lead_sig == FSINFO_LEAD_SIG
            && self.struc_sig == FSINFO_STRUC_SIG
            && self.trail_sig == FSINFO_TRAIL_SIG
    }

    /// Returns the next-free hint as a cluster id, or `None` if unknown/invalid.
    /// The hint must be >= 2 (first data cluster) to be useful.
    pub fn next_free_hint(&self) -> Option<u32> {
        if self.nxt_free == FSINFO_UNKNOWN || self.nxt_free < 2 {
            None
        } else {
            Some(self.nxt_free)
        }
    }
}
