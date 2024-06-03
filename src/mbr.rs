//! A simple Master Booot Record implementation.
//!
//! This can be used to read a MBR from a device. It's decoupled from vfat, so it could be used on its own.
use crate::{const_assert_size, error, BlockDevice, SectorId};
use binrw::io::Cursor;
use binrw::BinRead;
use binrw::BinReaderExt;
use log::error;

/// Magic indicating a valid bootsector
pub const VALID_BOOTSECTOR_SIGN: [u8; 2] = [0x55, 0xAA];

/// Look at PartitionEntry / bootable_indicator_flag.
pub const BOOTABLE_PARTITION_FLAG: u8 = 0x80;

/// From: https://en.wikipedia.org/wiki/Partition_type#List_of_partition_IDs
/// Used for identifying FAT32 partition type in the MBR headers
pub const FAT32_PARTITION_ID: [u8; 2] = [0xB, 0xC];

/// Always available in sector 0
/// packed is needed otherwise total size assert fails.
#[derive(Debug, Clone, BinRead)]
pub struct MasterBootRecord {
    /// MBRBootstrap(flat binary executable code)
    _mbr_bootstrap: [u8; 436],
    /// Optional "unique" disk ID
    pub disk_id: [u8; 10],
    /// MBRPartition Table, with 4 entries
    pub partitions: [PartitionEntry; 4],
    /// (0x55, 0xAA) "Valid bootsector" signature bytes - check `VALID_BOOTSECTOR_SIGN`
    pub valid_bootsector_sign: [u8; 2],
}
const_assert_size!(MasterBootRecord, 512);

impl From<[u8; 512]> for MasterBootRecord {
    fn from(input: [u8; 512]) -> Self {
        Cursor::new(&input).read_ne().unwrap()
    }
}

impl MasterBootRecord {
    /// Load a MBR from a device T.
    pub fn load<T: BlockDevice>(mut device: T) -> MasterBootRecord {
        let mut buff = [0; 512];
        device.read_sector(SectorId(0), &mut buff).unwrap();
        MasterBootRecord::from(buff)
    }
    /// Returns OK if index is a vfat partition.
    pub fn get_vfat_partition(&self, index: usize) -> error::Result<&PartitionEntry> {
        let partition = &self.partitions[index];
        if !FAT32_PARTITION_ID.contains(&partition.partition_type) {
            error!(
                "Requested partition index: {}, but partition's type is :{}",
                index, partition.partition_type
            );
            return Err(error::VfatRsError::Mbr {
                error: error::MbrError::InvalidPartition { index },
            });
        }
        Ok(partition)
    }
}

/// An entry in the MBR partition table
#[derive(Debug, Default, Clone, Copy, BinRead)]
pub struct PartitionEntry {
    /// Boot indicator bit flag: 0 = no, 0x80 = bootable (or "active")
    pub bootable_indicator_flag: u8,
    /// Starting head
    _starting_header: u8,
    /// Bits 6-7 are the upper two bits for the Starting Cylinder field.
    _starting_sector: u8,
    _starting_cylinder: u8,
    /// Partition Type (0xB or 0xC for FAT32).
    pub partition_type: u8,
    _ending_header: u8,
    _ending_sector: u8,
    _ending_cylinder: u8,
    /// Relative Sector (offset, in sectors, from start of disk to start of the partition)
    pub start_sector: u32,
    _total_sectors: u32,
}
