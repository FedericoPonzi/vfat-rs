use crate::api::raw_directory_entry::long_file_name_entry::LongFileNameEntry;
use crate::api::raw_directory_entry::{
    Attributes, EntryId, RegularDirectoryEntry, VfatDirectoryEntry,
};
use crate::{const_assert_eq, const_assert_size};
use core::mem;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct UnknownDirectoryEntry {
    pub(crate) id: u8,
    __unused: [u8; 10],
    /// Used to determine if a directory entry is an LFN entry.
    pub attributes: Attributes,
    __unused_after: [u8; 20],
}
const_assert_size!(UnknownDirectoryEntry, 32);

// Compile-time assertions that all transmuted types have identical sizes.
const_assert_eq!(
    core::mem::size_of::<UnknownDirectoryEntry>(),
    core::mem::size_of::<LongFileNameEntry>()
);
const_assert_eq!(
    core::mem::size_of::<UnknownDirectoryEntry>(),
    core::mem::size_of::<RegularDirectoryEntry>()
);

impl UnknownDirectoryEntry {
    /// Returns true if this entry is a Long File Name.
    pub(crate) fn is_lfn(&self) -> bool {
        self.attributes.is_lfn()
    }
    pub fn is_end_of_entries(&self) -> bool {
        let vfat_entry = VfatDirectoryEntry::from(self);
        matches!(vfat_entry, VfatDirectoryEntry::EndOfEntries(_))
    }
    pub fn is_deleted(&self) -> bool {
        let vfat_entry = VfatDirectoryEntry::from(self);
        matches!(vfat_entry, VfatDirectoryEntry::Deleted(_))
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
        // SAFETY: Both types are #[repr(C, packed)] with identical size (32 bytes).
        // They represent the same on-disk FAT32 directory entry format.
        // UnknownDirectoryEntry is a generic view of the same memory layout.
        // This is just reinterpreting between different views of the same data.
        unsafe { mem::transmute(lfn) }
    }
}

impl From<RegularDirectoryEntry> for UnknownDirectoryEntry {
    fn from(regular: RegularDirectoryEntry) -> Self {
        // SAFETY: Both types are #[repr(C, packed)] with identical size (32 bytes).
        // They represent the same on-disk FAT32 directory entry format.
        // UnknownDirectoryEntry is a generic view of the same memory layout.
        // This is just reinterpreting between different views of the same data.
        unsafe { mem::transmute(regular) }
    }
}

impl From<UnknownDirectoryEntry> for LongFileNameEntry {
    fn from(ue: UnknownDirectoryEntry) -> Self {
        // SAFETY: Both types are #[repr(C, packed)] with identical size (32 bytes).
        // They represent the same on-disk FAT32 directory entry format.
        // LongFileNameEntry is a specialized view of the same memory layout.
        // This is just reinterpreting between different views of the same data.
        unsafe { mem::transmute(ue) }
    }
}

impl From<UnknownDirectoryEntry> for RegularDirectoryEntry {
    fn from(ue: UnknownDirectoryEntry) -> Self {
        // SAFETY: Both types are #[repr(C, packed)] with identical size (32 bytes).
        // They represent the same on-disk FAT32 directory entry format.
        // RegularDirectoryEntry is a specialized view of the same memory layout.
        // This is just reinterpreting between different views of the same data.
        unsafe { mem::transmute(ue) }
    }
}

impl From<UnknownDirectoryEntry> for [u8; size_of::<UnknownDirectoryEntry>()] {
    fn from(entry: UnknownDirectoryEntry) -> Self {
        let mut buf = [0u8; 32];
        buf[0] = entry.id;
        buf[1..11].copy_from_slice(&entry.__unused);
        buf[11] = entry.attributes.0;
        buf[12..32].copy_from_slice(&entry.__unused_after);
        buf
    }
}

// todo: find a way to parametrize const T: usize

pub fn unknown_entry_convert_to_bytes_2(
    entries: [UnknownDirectoryEntry; 2],
) -> [u8; size_of::<UnknownDirectoryEntry>() * 2] {
    let mut result = [0u8; size_of::<UnknownDirectoryEntry>() * 2];
    for (i, entry) in entries.into_iter().enumerate() {
        let entry_bytes: [u8; 32] = entry.into();
        let start = i * size_of::<UnknownDirectoryEntry>();
        let end = start + size_of::<UnknownDirectoryEntry>();
        result[start..end].copy_from_slice(&entry_bytes);
    }
    result
}
impl From<[u8; size_of::<UnknownDirectoryEntry>()]> for UnknownDirectoryEntry {
    fn from(buf: [u8; size_of::<UnknownDirectoryEntry>()]) -> Self {
        Self {
            id: buf[0],
            __unused: buf[1..11].try_into().expect("slice with incorrect length"),
            attributes: Attributes(buf[11]),
            __unused_after: buf[12..32].try_into().expect("slice with incorrect length"),
        }
    }
}
