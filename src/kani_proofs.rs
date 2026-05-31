//! Formal verification harnesses for the Kani model checker.
//!
//! These proofs are only compiled under `cfg(kani)` (i.e. when running
//! `cargo kani`) and have no effect on normal builds or tests. They focus on
//! the crate's pure, bit-twiddling conversion logic, where bugs are easy to
//! introduce and hard to catch with example-based tests.

use crate::ClusterId;
use crate::fat_table::FatEntry;

/// The lower 28 bits hold the cluster value; the upper 4 bits are reserved and
/// must be ignored when decoding a FAT32 entry.
const FAT_ENTRY_VALUE_MASK: u32 = (1 << 28) - 1;

/// Decoding a raw FAT entry and re-encoding it to a `u32` preserves exactly the
/// lower 28 bits of the input, for every possible 4-byte input.
#[kani::proof]
fn fat_entry_decode_preserves_low_28_bits() {
    let buff: [u8; 4] = kani::any();
    let entry = FatEntry::from(buff);
    let encoded: u32 = entry.into();
    let raw = u32::from_le_bytes(buff);
    assert_eq!(encoded, raw & FAT_ENTRY_VALUE_MASK);
}

/// `FatEntry::new_ref` (slice-based) and `FatEntry::from` (array-based) decode
/// identically for any 4-byte input.
#[kani::proof]
fn fat_entry_new_ref_matches_from() {
    let buff: [u8; 4] = kani::any();
    assert_eq!(FatEntry::new_ref(&buff), FatEntry::from(buff));
}

/// Splitting a cluster id into its high/low 16-bit halves and reassembling it
/// round-trips for every possible cluster id.
#[kani::proof]
fn cluster_id_high_low_roundtrip() {
    let n: u32 = kani::any();
    let id = ClusterId::new(n);
    let (high, low) = id.into_high_low();
    assert_eq!(ClusterId::from_high_low(high, low), id);
}
