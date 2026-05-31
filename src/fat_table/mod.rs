pub(crate) use fat_entry::*;
pub(crate) use fat_reader::*;
pub(crate) use fat_writer::*;

use crate::VfatRsError::CheckedMulFailed;
use crate::cache::CachedPartition;
use crate::formats::cluster_id::ClusterId;
use crate::{SectorId, error};

mod fat_entry;
mod fat_reader;
mod fat_writer;

/// Given a cluster_id, returns the sector id to read to get the FAT table entry for
/// this cluster id.
fn get_params(device: &CachedPartition, cluster_id: ClusterId) -> error::Result<(SectorId, usize)> {
    fat_entry_location(
        u32::from(cluster_id),
        device.sector_size,
        device.fat_start_sector.0,
    )
}

/// Pure computation of the on-disk location (sector + byte offset) of a cluster's
/// FAT entry. Extracted from [`get_params`] so it can be reasoned about in
/// isolation (see `kani_proofs`): it takes untrusted on-disk values and must
/// never panic, and the returned offset must leave room for the 4-byte entry
/// within the sector.
pub(crate) fn fat_entry_location(
    cluster_id: u32,
    sector_size: usize,
    fat_start_sector: u32,
) -> error::Result<(SectorId, usize)> {
    // this should be 512 / 32 = 18
    let fat_entries_per_sector = sector_size / FAT_ENTRY_SIZE;
    if fat_entries_per_sector == 0 {
        return Err(error::VfatRsError::FilesystemCorrupted {
            reason: "sector size too small to hold a FAT entry",
        });
    }
    // In which sector is this cid contained? Cid: 222 / 18 = 12.
    // Done in usize to avoid truncating `fat_entries_per_sector` to 0 on a cast.
    let containing_sector = (cluster_id as usize / fat_entries_per_sector) as u32;
    // The sector is 12, now let's calculate the offset in that sector: 222 % 18 = 6.
    let offset_in_sector = (cluster_id as usize % fat_entries_per_sector)
        .checked_mul(FAT_ENTRY_SIZE)
        .ok_or(CheckedMulFailed)?;

    let sector = fat_start_sector.checked_add(containing_sector).ok_or(
        error::VfatRsError::FilesystemCorrupted {
            reason: "FAT entry sector index overflowed u32",
        },
    )?;

    Ok((SectorId(sector), offset_in_sector))
}
