use crate::const_assert_size;
use binrw::BinRead;

// TODO: Impl debug.
/// https://wiki.osdev.org/FAT#BPB_.28BIOS_Parameter_Block.29
#[derive(Debug, Copy, Clone, BinRead)]
#[repr(C, packed)]
pub struct BiosParameterBlock {
    /// These bytes are EB XX 90 -> JMP SHORT XX NOP
    _jump_instr: [u8; 3],
    /// OEM identifier. The first 8 Bytes (3 - 10) is the version of DOS being used. The next eight Bytes 29 3A 63 7E 2D 49 48 and 43 read out the name of the version.
    /// The official FAT Specification from Microsoft says that this field is really meaningless and is ignored by MS FAT Drivers,
    /// however it does recommend the value "MSWIN4.1" as some 3rd party drivers supposedly check it and expect it to have that value.
    /// Older versions of dos also report MSDOS5.1, linux-formatted floppy will likely to carry "mkdosfs" here, and FreeDOS formatted disks have been observed to have "FRDOS5.1" here.
    /// If the string is less than 8 bytes, it is padded with spaces.
    oem_identifier: u64,
    /// Number of bytes per sector, in little-endian format
    pub bytes_per_sector: u16,
    /// Numbr of sectors per cluster:
    pub sectors_per_cluster: u8,
    /// Number of reserved sectors. Includes boot records.
    pub reserved_sectors: u16,
    /// Number of File Allocation Tables (FAT's) on the storage media. Often 2
    pub fat_amount: u8,
    max_num_directory_entries: u16,
    // Total logical sectors (if zero, use total_logical_sectors_gt_u16 field instead)
    total_logical_sectors: u16,
    fat_id: u8,
    /// Number of sectors per FAT. 0 for FAT32; use 32-bit value in extended bpb instead
    pub sectors_per_fat: u16,
    num_sectors_per_track: u16,
    num_heads_on_storage: u16,
    num_hidden_sectors: u32,
    /// Total logical sectors if greater than 65535; otherwise, see num_sectors_per_fat.
    pub total_logical_sectors_gt_u16: u32,
}
const_assert_size!(BiosParameterBlock, 36);

#[derive(Debug, Copy, Clone, BinRead)]
pub struct ExtendedBiosParameterBlock {
    pub sectors_per_fat: u32,
    _flags: u16,
    _fat_version: u16,
    /// Cluster pointing to the root (`/`) directory
    pub root_cluster: u32,
    pub(crate) fsinfo_sector: u16,
    _backup_boot_sector: u16,
    _reserved: [u8; 12],
    _drive_number: u8,
    _reserved2: u8,
    /// 0x28 or 0x29 for VFat / fat 32.
    pub signature: u8,
    _volumeid_serial_number: u32,
    /// Padded with spaces.
    pub volume_label_string: [u8; 11],
    /// System identifier string. This field is a string representation of the FAT file system type.
    /// It is padded with spaces.
    /// The spec says never to trust the contents of this string for any use.
    _system_identifier_string: [u8; 8],
    _boot_code: [u8; 420],
    /// https://stackoverflow.com/questions/1125025/what-is-the-role-of-magic-number-in-boot-loading-in-linux
    _bootable_partition_signature: u16,
}

const_assert_size!(ExtendedBiosParameterBlock, 476);

#[derive(Debug, Clone, BinRead)]
pub struct FullExtendedBIOSParameterBlock {
    pub bpb: BiosParameterBlock,
    pub extended: ExtendedBiosParameterBlock,
}
impl FullExtendedBIOSParameterBlock {
    pub fn get_fat_size(&self) -> u32 {
        self.extended.sectors_per_fat * self.bpb.bytes_per_sector as u32
    }
    pub fn sectors_occupied_by_all_fats(&self) -> u32 {
        self.bpb.fat_amount as u32 * self.bpb.sectors_per_fat as u32
    }
}

//const_assert_size!(FullExtendedBIOSParameterBlock, 512);
