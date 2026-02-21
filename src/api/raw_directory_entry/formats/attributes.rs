use crate::const_assert_size;
use core::fmt;
use core::fmt::Debug;

/// FAT directory entry attribute constants.
pub mod attribute {
    /// Read-only file.
    pub const READ_ONLY: u8 = 0x01;
    /// Hidden file.
    pub const HIDDEN: u8 = 0x02;
    /// System file.
    pub const SYSTEM: u8 = 0x04;
    /// Volume ID label.
    pub const VOLUME_ID: u8 = 0x08;
    /// Directory entry.
    pub const DIRECTORY: u8 = 0x10;
    /// Archive flag.
    pub const ARCHIVE: u8 = 0x20;
    /// Long file name entry (combination of READ_ONLY | HIDDEN | SYSTEM | VOLUME_ID).
    pub const LFN: u8 = READ_ONLY | HIDDEN | SYSTEM | VOLUME_ID;
}

/// A FAT directory entry's attributes byte.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Attributes(pub u8);
impl Attributes {
    /// Create attributes for a new directory entry.
    pub fn new_directory() -> Self {
        Self(attribute::DIRECTORY)
    }
    fn matches(&self, attribute: u8) -> bool {
        self.0 & attribute == attribute
    }
    /// Returns `true` if this is a long file name entry.
    pub fn is_lfn(&self) -> bool {
        self.matches(attribute::LFN)
    }
    /// Returns `true` if the read-only bit is set.
    pub fn is_read_only(&self) -> bool {
        self.matches(attribute::READ_ONLY)
    }
    /// Returns `true` if the hidden bit is set.
    pub fn is_hidden(&self) -> bool {
        self.matches(attribute::HIDDEN)
    }
    /// Returns `true` if the system bit is set.
    pub fn is_system(&self) -> bool {
        self.matches(attribute::SYSTEM)
    }
    /// Returns `true` if this is a volume ID label.
    pub fn is_volume_id(&self) -> bool {
        self.matches(attribute::VOLUME_ID)
    }
    /// Returns `true` if this is a directory.
    pub fn is_directory(&self) -> bool {
        self.matches(attribute::DIRECTORY)
    }
    /// Returns `true` if the archive bit is set.
    pub fn is_archive(&self) -> bool {
        self.matches(attribute::ARCHIVE)
    }
}
impl Debug for Attributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Attributes(")?;
        if self.is_lfn() {
            // if is a longfilename, no need to print other fields.
            return write!(f, "LFN)");
        }
        if self.is_read_only() {
            write!(f, "READ_ONLY, ")?;
        }
        if self.is_hidden() {
            write!(f, "HIDDEN, ")?;
        }
        if self.is_system() {
            write!(f, "SYSTEM, ")?;
        }
        if self.is_volume_id() {
            write!(f, "VOLUME_ID")?;
        }
        if self.is_directory() {
            write!(f, "DIRECTORY, ")?;
        }
        if self.is_archive() {
            write!(f, "ARCHIVE, ")?;
        }
        write!(f, ")")
    }
}

const_assert_size!(Attributes, 1);
