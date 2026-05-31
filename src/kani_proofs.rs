//! Formal verification harnesses for the Kani model checker.
//!
//! These proofs are only compiled under `cfg(kani)` (i.e. when running
//! `cargo kani`) and have no effect on normal builds or tests.
//!
//! They target logic that operates on *untrusted on-disk values* and is easy to
//! get subtly wrong:
//!
//! * the `defbit!`-generated bitfield accessors used to pack/unpack every packed
//!   on-disk structure (timestamps, LFN sequence numbers, ...), where a bug
//!   could let writing one field silently corrupt an adjacent one; and
//! * the address arithmetic that maps a (corruptible) cluster id to the sector +
//!   byte offset of its FAT entry, which must never panic and must keep the
//!   4-byte read inside the sector.

use crate::api::timestamp::VfatTimestamp;
use crate::fat_table::{FAT_ENTRY_SIZE, FatEntry, fat_entry_location};

// ---------------------------------------------------------------------------
// `defbit!` bitfield accessors (exercised through `VfatTimestamp`)
// ---------------------------------------------------------------------------

/// `set_value` must only ever touch bits inside its mask: every bit outside the
/// mask is preserved, for any starting state, any value, and any (non-empty)
/// mask. This is what guarantees that writing one packed field can never corrupt
/// a neighbour. Masks are always non-zero field constants by construction.
#[kani::proof]
fn bitfield_set_value_confined_to_mask() {
    let state: u32 = kani::any();
    let val: u32 = kani::any();
    let mask: u32 = kani::any();
    kani::assume(mask != 0);

    let mut ts = VfatTimestamp::new(state);
    ts.set_value(val, mask);

    assert_eq!(ts.get() & !mask, state & !mask);
}

/// Round-trip: writing an in-range value to a field and reading it back returns
/// exactly that value, regardless of the surrounding bits. Uses the real `MONTH`
/// field (bounded above and below by other fields).
#[kani::proof]
fn bitfield_roundtrip_preserves_in_range_value() {
    let state: u32 = kani::any();
    let val: u32 = kani::any();

    let mask = VfatTimestamp::MONTH;
    let field_max = mask >> mask.trailing_zeros();
    kani::assume(val <= field_max);

    let mut ts = VfatTimestamp::new(state);
    ts.set_value(val, mask);

    assert_eq!(ts.get_value(mask), val);
}

/// Writing one field must not change the decoded value of its neighbours.
#[kani::proof]
fn bitfield_set_field_preserves_neighbours() {
    let state: u32 = kani::any();
    let val: u32 = kani::any();

    let mut ts = VfatTimestamp::new(state);
    let year_before = ts.get_value(VfatTimestamp::YEAR);
    let day_before = ts.get_value(VfatTimestamp::DAY);

    ts.set_value(val, VfatTimestamp::MONTH);

    assert_eq!(ts.get_value(VfatTimestamp::YEAR), year_before);
    assert_eq!(ts.get_value(VfatTimestamp::DAY), day_before);
}

/// A decoded field value always fits within the field's bit width, for any
/// (possibly corrupt) on-disk state.
#[kani::proof]
fn bitfield_get_value_fits_in_field() {
    let state: u32 = kani::any();
    let ts = VfatTimestamp::new(state);

    let mask = VfatTimestamp::YEAR;
    assert!(ts.get_value(mask) <= mask >> mask.trailing_zeros());
}

// ---------------------------------------------------------------------------
// FAT entry address arithmetic (untrusted cluster ids / BPB values)
// ---------------------------------------------------------------------------

/// For *any* cluster id, sector size, and FAT start sector, locating a FAT entry
/// must never panic (overflow is reported as an error instead), and whenever it
/// succeeds the byte offset must leave room for the full 4-byte entry inside the
/// sector — otherwise the subsequent `read_sector_offset` would read out of
/// bounds.
#[kani::proof]
fn fat_entry_location_never_panics_and_offset_in_bounds() {
    let cluster_id: u32 = kani::any();
    let sector_size: usize = kani::any();
    let fat_start_sector: u32 = kani::any();

    if let Ok((_sector, offset)) = fat_entry_location(cluster_id, sector_size, fat_start_sector) {
        assert!(offset + FAT_ENTRY_SIZE <= sector_size);
    }
}

/// On the valid domain (real 512-byte sectors, in-range cluster ids and FAT
/// start) the refactored helper still matches the original FAT32 formula.
#[kani::proof]
fn fat_entry_location_matches_formula() {
    let cluster_id: u32 = kani::any();
    let fat_start_sector: u32 = kani::any();

    let sector_size: usize = 512;
    // Largest valid FAT32 cluster id is 28-bit; keep the FAT start small enough
    // that the sector index cannot overflow, so we exercise the success path.
    kani::assume(cluster_id <= 0x0FFF_FFFF);
    kani::assume(fat_start_sector <= 0x000F_FFFF);

    let entries_per_sector = (sector_size / FAT_ENTRY_SIZE) as u32;
    let (sector, offset) = fat_entry_location(cluster_id, sector_size, fat_start_sector).unwrap();

    assert_eq!(sector.0, fat_start_sector + cluster_id / entries_per_sector);
    assert_eq!(
        offset,
        (cluster_id % entries_per_sector) as usize * FAT_ENTRY_SIZE
    );
}

// ---------------------------------------------------------------------------
// FAT entry decoding (untrusted 4-byte on-disk values)
// ---------------------------------------------------------------------------

/// Decoding any raw 4-byte FAT entry and re-encoding it preserves exactly the
/// lower 28 bits (the upper 4 bits are reserved and must be ignored), and never
/// panics — for every possible input.
#[kani::proof]
fn fat_entry_decode_preserves_low_28_bits() {
    let buff: [u8; 4] = kani::any();
    let entry = FatEntry::from(buff);
    let encoded: u32 = entry.into();
    let raw = u32::from_le_bytes(buff);

    const FAT_ENTRY_VALUE_MASK: u32 = (1 << 28) - 1;
    assert_eq!(encoded, raw & FAT_ENTRY_VALUE_MASK);
}
