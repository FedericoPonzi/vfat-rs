use alloc::format;
use alloc::string::String;
use core::fmt;
use core::fmt::{Debug, Formatter};

use crate::api::directory_entry::{Attributes, VfatDirectoryEntry};
use crate::api::timestamp::{Milliseconds, VfatTimestamp};
use crate::api::Metadata;
use crate::{const_assert_size, ClusterId};

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct RegularDirectoryEntry {
    ///File name: 8 ASCII characters.
    /// A file name may be terminated early using 0x00or 0x20 characters.
    /// If the file name starts with 0x00, the previous entry was the last entry.
    /// If the file name starts with 0xE5, this is a deleted/unused entry.
    pub file_name: [u8; 8],
    /// Extension of the file, 8 ASCII characters.
    pub file_ext: [u8; 3],
    /// Attributes of this file.
    pub(crate) attributes: Attributes,
    /// Reserved for future use
    pub(crate) _reseverd_win_nt: u8,
    /// Creation time's milliseconds
    pub(crate) creation_millis: Milliseconds,
    /// Creation time using VfatTimestamp format. Coalesces creation date + creation time.
    pub creation_time: VfatTimestamp,
    /// Last access date. Same format as the creation date.
    pub last_access_date: u16,
    /// Higher 16bits of the file's ClusterId
    pub high_16bits: u16,
    /// Last modification date and time.
    pub last_modification_time: VfatTimestamp,
    /// Lower 16 bits of the file's ClusterId.
    pub low_16bits: u16,
    /// The size of the file in bytes.
    pub file_size: u32,
}
const_assert_size!(RegularDirectoryEntry, 32);

impl Debug for RegularDirectoryEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("RegularDirectoryEntry")
            .field(
                "file_name",
                &format_args!("{}", String::from_utf8_lossy(&{ self.file_name })),
            )
            .field(
                "file_ext",
                &format_args!("{}", String::from_utf8_lossy(&{ self.file_ext })),
            )
            .field("attributes", &{ self.attributes })
            .field("file_size", &{ self.file_size })
            .field("low-high", &format_args!("{:?}", self.cluster()))
            .finish()
    }
}

impl From<Metadata> for RegularDirectoryEntry {
    fn from(metadata: Metadata) -> Self {
        let file_name = VfatDirectoryEntry::regular_filename_from(metadata.name());
        let file_ext = VfatDirectoryEntry::get_regular_filename_ext(metadata.name());
        let (high_16bits, low_16bits) = metadata.cluster.into_high_low();
        RegularDirectoryEntry {
            file_name,
            file_ext,
            high_16bits,
            low_16bits,
            attributes: metadata.attributes,
            creation_time: metadata.creation().unwrap(),
            last_modification_time: metadata.last_update().unwrap(),
            file_size: metadata.size,
            _reseverd_win_nt: 0,
            creation_millis: Default::default(),
            last_access_date: 0,
        }
    }
}

impl RegularDirectoryEntry {
    pub fn is_dir(&self) -> bool {
        self.attributes.is_directory()
    }
    pub(crate) fn cluster(&self) -> ClusterId {
        ClusterId::from_high_low(self.high_16bits, self.low_16bits)
    }
    pub fn is_volume_id(&self) -> bool {
        self.attributes.is_volume_id()
    }
    pub fn is_lfn(&self) -> bool {
        self.attributes.is_lfn()
    }
    /// Handles everything needed for returning a correct name.
    pub fn full_name(&self) -> String {
        let name = String::from_utf8_lossy(self.file_name());
        let ext = self
            .extension()
            .map(|ext| format!(".{}", String::from_utf8_lossy(ext)))
            .unwrap_or_default();
        format!("{}{}", name, ext)
    }
    fn early_terminate_pos(v: &[u8]) -> usize {
        for (pos, ch) in v.iter().enumerate() {
            if *ch == 0x00 || *ch == 0x20 {
                return pos;
            }
        }
        v.len()
    }
    pub fn file_name(&self) -> &[u8] {
        let pos = Self::early_terminate_pos(&self.file_name);
        &self.file_name[..pos]
    }
    pub fn extension(&self) -> Option<&[u8]> {
        let pos = Self::early_terminate_pos(&self.file_ext);
        (pos > 0).then(|| &self.file_ext[..pos])
    }
}
