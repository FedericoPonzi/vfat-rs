use crate::api::raw_directory_entry::long_file_name_entry::LongFileNameEntry;
use crate::api::raw_directory_entry::{
    Attributes, EntryId, RegularDirectoryEntry, VfatDirectoryEntry,
};
use crate::const_assert_size;
use core::mem;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct UnknownDirectoryEntry {
    pub(crate) id: u8,
    __unused: [u8; 10],
    /// Used to determine if a directory entry is an LFN entry.
    pub attributes: Attributes,
    __unused_after: [u8; 20],
}
const_assert_size!(UnknownDirectoryEntry, 32);
impl UnknownDirectoryEntry {
    /// Returns true if this entry is a Long File Name.
    pub(crate) fn is_lfn(&self) -> bool {
        self.attributes.is_lfn()
    }
    pub fn is_end_of_entries(&self) -> bool {
        let vfat_entry = VfatDirectoryEntry::from(self);
        matches!(vfat_entry, VfatDirectoryEntry::EndOfEntries(_))
    }
    pub fn last_entry(&self) -> bool {
        self.is_end_of_entries()
    }
    pub fn set_id(&mut self, entry_id: EntryId) {
        self.id = entry_id.into();
    }
}
impl From<LongFileNameEntry> for UnknownDirectoryEntry {
    fn from(lfn: LongFileNameEntry) -> Self {
        unsafe { mem::transmute(lfn) }
    }
}

impl From<RegularDirectoryEntry> for UnknownDirectoryEntry {
    fn from(regular: RegularDirectoryEntry) -> Self {
        unsafe { mem::transmute(regular) }
    }
}

impl From<UnknownDirectoryEntry> for LongFileNameEntry {
    fn from(ue: UnknownDirectoryEntry) -> Self {
        unsafe { mem::transmute(ue) }
    }
}

impl From<UnknownDirectoryEntry> for RegularDirectoryEntry {
    fn from(ue: UnknownDirectoryEntry) -> Self {
        unsafe { mem::transmute(ue) }
    }
}

impl From<UnknownDirectoryEntry> for [u8; mem::size_of::<UnknownDirectoryEntry>()] {
    fn from(ude: UnknownDirectoryEntry) -> Self {
        unsafe { mem::transmute(ude) }
    }
}

// todo: find a way to parametrize const T: usize

pub fn unknown_entry_convert_to_bytes_2(
    entries: [UnknownDirectoryEntry; 2],
) -> [u8; mem::size_of::<UnknownDirectoryEntry>() * 2] {
    unsafe { mem::transmute(entries) }
}
impl From<[u8; mem::size_of::<UnknownDirectoryEntry>()]> for UnknownDirectoryEntry {
    fn from(buf: [u8; mem::size_of::<UnknownDirectoryEntry>()]) -> Self {
        unsafe { mem::transmute(buf) }
    }
}
