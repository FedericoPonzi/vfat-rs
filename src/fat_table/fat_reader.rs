use crate::ArcMutex;
use crate::error::Result;
use crate::fat_table::fat_entry::FAT_ENTRY_SIZE;
use crate::fat_table::{FatEntry, get_params};
use crate::{CachedPartition, ClusterId};

/// Returns the next clusterid in the chain after the provided cluster_id, if any.
/// To do that, query the fat table for this cluster id, and see if it is a Data Cluster (e.g. a
/// node in the chain) return the next element, otherwise it's a dead end and return null.
pub(crate) fn next_cluster(
    cluster_id: ClusterId,
    device: ArcMutex<CachedPartition>,
) -> Result<Option<ClusterId>> {
    let fat_entry = read_fat_entry(cluster_id, device)?;
    Ok(match fat_entry {
        FatEntry::DataCluster(id) => Some(ClusterId::new(id)),
        _ => None,
    })
}

pub(crate) fn read_fat_entry(
    cluster_id: ClusterId,
    device: ArcMutex<CachedPartition>,
) -> Result<FatEntry> {
    let mut buf = [0u8; FAT_ENTRY_SIZE];
    let (sector, offset) = get_params(&device, cluster_id)?;
    device
        .read_sector_offset(sector, offset, &mut buf)
        .map(|_| FatEntry::from(buf))
}
