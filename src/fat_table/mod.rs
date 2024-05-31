pub(crate) use fat_entry::*;
pub(crate) use fat_reader::*;
pub(crate) use fat_writer::*;

use crate::cache::CachedPartition;
use crate::formats::cluster_id::ClusterId;
use crate::VfatRsError::CheckedMulFailed;
use crate::{error, SectorId};

mod fat_entry;
mod fat_reader;
mod fat_writer;

/// Given a cluster_id, returns the sector id to read to get the FAT table entry for
/// this cluster id.
fn get_params(device: &CachedPartition, cluster_id: ClusterId) -> error::Result<(SectorId, usize)> {
    // this should be 512 / 32 = 18
    let fat_entries_per_sector = device.sector_size / FAT_ENTRY_SIZE;
    // In which sector is this cid contained? Cid: 222 / 18 = 12.3333

    let containing_sector = (f64::from(cluster_id) / fat_entries_per_sector as f64).floor() as u32;
    // The sector is 12, now let's calculate the offset in that sector: 222 % 18 = 6.

    let offset_in_sector = ((f64::from(cluster_id) % fat_entries_per_sector as f64).floor()
        as usize)
        .checked_mul(FAT_ENTRY_SIZE)
        .ok_or(CheckedMulFailed)?;

    let sector = SectorId(device.fat_start_sector + containing_sector);

    Ok((sector, offset_in_sector))
}
