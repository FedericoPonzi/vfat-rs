use crate::ClusterId;

pub(crate) const FAT_ENTRY_SIZE: usize = size_of::<u32>();

/// A fat32 row entry. Each entry represents a cluster. This is the "high level" view
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
#[repr(C)]
pub(crate) enum FatEntry {
    /// Entry 0, formatted as 0xFFFFFFFN
    #[allow(dead_code)]
    Id(u32),
    /// A free, unused cluster. 0x00
    #[default]
    Unused,
    /// 0x01: reserved
    Reserved(u32),
    /// A data cluster; value points to next cluster in chain.
    DataCluster(u32),
    /// Last cluster in chain. Should be, but may not be, the EndOfChainMarker (e.g. entry 1).
    LastCluster(u32),
}

impl FatEntry {
    pub(crate) fn from_chain(next: ClusterId) -> Self {
        Self::DataCluster(next.into())
    }
    pub fn as_buff(self) -> [u8; FAT_ENTRY_SIZE] {
        let raw_fat: u32 = self.into();
        raw_fat.to_le_bytes()
    }

    pub fn new_ref(buff: &[u8]) -> Self {
        let mut temp = [0u8; 4];
        temp.copy_from_slice(buff);
        Self::from(temp)
    }
}
impl From<[u8; FAT_ENTRY_SIZE]> for FatEntry {
    fn from(buff: [u8; FAT_ENTRY_SIZE]) -> Self {
        let val = u32::from_le_bytes(buff);
        use FatEntry::*;
        let lower_28_bits_mask: u32 = (1 << 28) - 1;
        // The upper 4 bits are ignored.
        let val = val & lower_28_bits_mask;
        match val {
            0x0 => Unused,
            //0x1 => Reserved(val),
            0x0000002..=0xFFFFFEF => DataCluster(val),
            //0xFFFFFF0..=0xFFFFFF7 => Reserved(val),
            0xFFFFFF8..=0xFFFFFFF => LastCluster(val),
            val => Reserved(val),
        }
    }
}

impl From<FatEntry> for u32 {
    fn from(fat_entry: FatEntry) -> Self {
        use FatEntry::*;
        match fat_entry {
            Unused => 0x0,
            Reserved(i) => i,
            DataCluster(i) => i,
            LastCluster(i) => i,
            Id(i) => i,
        }
    }
}
