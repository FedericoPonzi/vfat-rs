use alloc::string::String;
use snafu::prelude::*;

/// VfatRS result type
pub type Result<T> = core::result::Result<T, VfatRsError>;
use crate::io::Error as IoError;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum VfatRsError {
    #[snafu(display("MBR Error: {error}"))]
    Mbr { error: MbrError },
    #[snafu(display("Free cluster not found, probably memory is full!?"))]
    FreeClusterNotFound,
    #[snafu(display("Checked mult failed."))]
    CheckedMulFailed,
    #[snafu(display("An entry (file/directory) named '{}' already exists.", target))]
    NameAlreadyInUse { target: String },
    #[snafu(display("Io Error: {}", source))]
    IoError { source: IoError },
    #[snafu(display("Unsupported vfat partition found, signature: {}", target))]
    InvalidVfat { target: u8 },
    #[snafu(display(
        "Impossible delete non empty directory: {}. Contains: [{}]",
        target,
        contents
    ))]
    NonEmptyDirectory { target: String, contents: String },
    #[snafu(display("File not found: '{}'", target))]
    FileNotFound { target: String },
    #[snafu(display("Entry not found: '{}'", target))]
    EntryNotFound { target: String },
    #[snafu(display("Cannot delete pseudo directory: '{}'", target))]
    CannotDeletePseudoDir { target: String },
    #[snafu(display("Path '{}' is not absolute.", target))]
    PathNotAbsolute { target: String },
}

impl From<IoError> for VfatRsError {
    fn from(err: IoError) -> Self {
        VfatRsError::IoError { source: err }
    }
}

impl From<crate::io::ErrorKind> for VfatRsError {
    fn from(value: crate::io::ErrorKind) -> Self {
        VfatRsError::from(crate::io::Error::from(value))
    }
}

#[derive(Debug, Snafu)]
pub enum MbrError {
    #[snafu(display("Not a fat32 partition: {index}"))]
    InvalidPartition { index: usize },
}

// Used for Impl Write/Read
impl From<VfatRsError> for binrw::io::Error {
    fn from(_err: VfatRsError) -> Self {
        // TODO: provide useful output
        binrw::io::ErrorKind::Other.into()
    }
}

impl From<binrw::Error> for VfatRsError {
    fn from(err: binrw::Error) -> Self {
        // todo
        let kind = crate::io::ErrorKind::Other;
        match err {
            binrw::Error::Io(_err) => Self::from(IoError::new(kind, "IoError")),
            _ => {
                panic!("todo.")
            }
        }
    }
}
#[cfg(not(feature = "std"))]
impl From<binrw::io::Error> for VfatRsError {
    fn from(_err: binrw::io::Error) -> Self {
        // todo
        let kind = crate::io::ErrorKind::Other;
        Self::from(IoError::new(kind, "IoError"))
    }
}
