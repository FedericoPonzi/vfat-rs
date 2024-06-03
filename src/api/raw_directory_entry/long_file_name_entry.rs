use crate::api::raw_directory_entry::Attributes;
use crate::const_assert_size;
use crate::defbit;
use alloc::string::String;
use core::fmt;
use core::fmt::{Debug, Formatter};

// Sequence Number
// Bit 6 set: last logical LFN entry.
// Bit 5 clear: first physical LFN entry
// Bits 4-0: from 0x01..0x14(0x1F): position of entry
// If the sequence number is 0x00, the previous entry was the last entry.
// If the sequence number is 0xE5, this is a deleted/unused entry.
defbit!(
    SequenceNumber,
    u8,
    [LastLogical[6 - 6], FirstPhysical[5 - 5], Position[4 - 0],]
);
impl SequenceNumber {
    pub fn set_first_physical_bit(&mut self) {
        self.set_bit(SequenceNumber::FirstPhysical);
    }

    pub fn get_position(&self) -> u8 {
        self.get_value(SequenceNumber::Position)
    }

    pub fn set_is_last_bit(&mut self) {
        self.set_bit(SequenceNumber::LastLogical);
    }
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
/// These special entries should not confuse old programs,
/// since they get the 0xf (read only / hidden / system / volume label) attribute combination
/// that should make sure that all old programs will ignore them.
pub struct LongFileNameEntry {
    /// Bit 6 set: last logical LFN entry.
    /// Bit 5 clear: first physical LFN entry
    /// Bits 4-0: from 0x01..0x14(0x1F): position of entry
    /// If the sequence number is 0x00, the previous entry was the last entry.
    /// If the sequence number is 0xE5, this is a deleted/unused entry.
    pub sequence_number: SequenceNumber,
    /// Name characters (fiveUCS-2(subset of UTF-16) characters)
    /// A file name may be terminated early using 0x00or 0xFF characters.
    pub name_characters: [u16; 5],
    /// Attributes (always0x0F).
    /// Used to determine if a directory entry is an LFN entry.attributes: Attributes,
    pub attributes: Attributes,
    /// Type (always0x00for VFAT LFN, other values reserved for future use;
    /// for special usage of bits 4 and 3 in SFNs see further up)
    pub r#type: u8,
    /// Checksum of DOS file name.
    pub(crate) checksum_dos_filename: u8,
    /// Second set of name characters (sixUCS-2characters).
    /// Same early termination conditions apply
    pub second_set_name: [u16; 6],
    /// Always0x0000 for an LFN
    pub(crate) _reserved: u16,
    /// Third set of name characters (two UCS-2characters).
    /// Same early termination conditions apply.
    pub third_set_name: [u16; 2],
}
const_assert_size!(LongFileNameEntry, 32);

impl Debug for LongFileNameEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name_characters = { self.name_characters };
        let second_set_name = { self.second_set_name };
        let third_set_name = { self.third_set_name };
        let attributes = { self.attributes };
        let checksum_dos_filename = { self.checksum_dos_filename };
        f.debug_struct("LongFileNameEntry")
            .field("sequence_number", &self.sequence_number)
            .field(
                "name_characters",
                &format_args!("{}", String::from_utf16_lossy(&name_characters)),
            )
            .field(
                "second_set_name",
                &format_args!("{}", String::from_utf16_lossy(&second_set_name)),
            )
            .field(
                "third_set_name",
                &format_args!("{}", String::from_utf16_lossy(&third_set_name)),
            )
            .field("attributes", &attributes)
            .field("checksum", &checksum_dos_filename)
            .finish()
    }
}

impl LongFileNameEntry {
    pub fn is_lfn(&self) -> bool {
        self.attributes.is_lfn()
    }

    // Returns a subset of the input, early stopping according to LFN name rules.
    fn early_terminate_pos(name_array: &[u16]) -> &[u16] {
        let get_pos = |string: &[u16]| {
            for (pos, ch) in string.iter().enumerate() {
                if *ch == 0x00 || *ch == 0xFFFF {
                    return pos;
                }
            }
            string.len()
        };
        &name_array[..get_pos(name_array)]
    }

    pub fn collect_name(&self) -> String {
        let name_characters = { self.name_characters };
        let second_set_name = { self.second_set_name };
        let third_set_name = { self.third_set_name };
        alloc::format!(
            "{}{}{}",
            String::from_utf16_lossy(Self::early_terminate_pos(&name_characters)),
            String::from_utf16_lossy(Self::early_terminate_pos(&second_set_name)),
            String::from_utf16_lossy(Self::early_terminate_pos(&third_set_name)),
        )
    }
    /// If the sequence number is 0x00, the previous entry was the last entry.
    pub fn was_last_entry_last(&self) -> bool {
        self.sequence_number.0 == 0x00
    }
}
