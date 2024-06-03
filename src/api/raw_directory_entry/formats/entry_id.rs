use crate::api::raw_directory_entry::{ID_DELETED_UNUSED_ENTRY, ID_LAST_ENTRY_WAS_LAST};

/// The first byte of an entry is called ID.
pub enum EntryId {
    Deleted,
    EndOfEntries,
    Valid(u8),
}
impl From<u8> for EntryId {
    fn from(id: u8) -> Self {
        match id {
            ID_LAST_ENTRY_WAS_LAST => Self::EndOfEntries,
            ID_DELETED_UNUSED_ENTRY => Self::Deleted,
            _ => Self::Valid(id),
        }
    }
}

impl From<EntryId> for u8 {
    fn from(entry_id: EntryId) -> Self {
        match entry_id {
            EntryId::EndOfEntries => ID_LAST_ENTRY_WAS_LAST,
            EntryId::Deleted => ID_DELETED_UNUSED_ENTRY,
            EntryId::Valid(id) => id,
        }
    }
}
