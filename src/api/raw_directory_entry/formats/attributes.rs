use crate::const_assert_size;
use core::fmt;
use core::fmt::Debug;

pub mod attribute {
    pub const READ_ONLY: u8 = 0x01;
    pub const HIDDEN: u8 = 0x02;
    pub const SYSTEM: u8 = 0x04;
    pub const VOLUME_ID: u8 = 0x08;
    pub const DIRECTORY: u8 = 0x10;
    pub const ARCHIVE: u8 = 0x20;
    pub const LFN: u8 = READ_ONLY | HIDDEN | SYSTEM | VOLUME_ID;
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Attributes(pub u8);
impl Attributes {
    pub fn new_directory() -> Self {
        Self(attribute::DIRECTORY)
    }
    fn matches(&self, attribute: u8) -> bool {
        self.0 & attribute == attribute
    }
    pub fn is_lfn(&self) -> bool {
        self.matches(attribute::LFN)
    }
    pub fn is_read_only(&self) -> bool {
        self.matches(attribute::READ_ONLY)
    }
    pub fn is_hidden(&self) -> bool {
        self.matches(attribute::HIDDEN)
    }
    pub fn is_system(&self) -> bool {
        self.matches(attribute::SYSTEM)
    }
    pub fn is_volume_id(&self) -> bool {
        self.matches(attribute::VOLUME_ID)
    }
    pub fn is_directory(&self) -> bool {
        self.matches(attribute::DIRECTORY)
    }
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
