use alloc::string::String;
use snafu::prelude::*;

/// VfatRS result type
pub type Result<T> = core::result::Result<T, VfatRsError>;
use crate::io::Error as IoError;

/// Errors that can occur during VFAT filesystem operations.
#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum VfatRsError {
    /// An MBR-related error.
    #[snafu(display("MBR Error: {error}"))]
    Mbr {
        /// The underlying MBR error.
        error: MbrError,
    },
    /// No free cluster available (disk full).
    #[snafu(display("Free cluster not found, probably memory is full!?"))]
    FreeClusterNotFound,
    /// An arithmetic overflow occurred.
    #[snafu(display("Checked mult failed."))]
    CheckedMulFailed,
    /// An entry with the given name already exists.
    #[snafu(display("An entry (file/directory) named '{}' already exists.", target))]
    NameAlreadyInUse {
        /// Name that collided.
        target: String,
    },
    /// An I/O error from the underlying block device.
    #[snafu(display("Io Error: {}", source))]
    IoError {
        /// The underlying I/O error.
        source: IoError,
    },
    /// The partition does not have a valid VFAT signature.
    #[snafu(display("Unsupported vfat partition found, signature: {}", target))]
    InvalidVfat {
        /// The invalid signature byte.
        target: u8,
    },
    /// Cannot delete a non-empty directory.
    #[snafu(display(
        "Impossible delete non empty directory: {}. Contains: [{}]",
        target,
        contents
    ))]
    NonEmptyDirectory {
        /// Directory name.
        target: String,
        /// Comma-separated list of remaining entries.
        contents: String,
    },
    /// The requested file was not found.
    #[snafu(display("File not found: '{}'", target))]
    FileNotFound {
        /// File name that was not found.
        target: String,
    },
    /// The requested entry was not found.
    #[snafu(display("Entry not found: '{}'", target))]
    EntryNotFound {
        /// Entry name that was not found.
        target: String,
    },
    /// Cannot delete the `.` or `..` pseudo-directories.
    #[snafu(display("Cannot delete pseudo directory: '{}'", target))]
    CannotDeletePseudoDir {
        /// Pseudo-directory name.
        target: String,
    },
    /// Moving a directory into its own subtree would create a cycle.
    #[snafu(display(
        "Cannot move directory '{}' into its own subdirectory '{}'",
        source_path,
        destination_path
    ))]
    CircularMove {
        /// Source directory path.
        source_path: String,
        /// Destination path that is inside the source.
        destination_path: String,
    },
    /// The supplied path is not absolute.
    #[snafu(display("Path '{}' is not absolute.", target))]
    PathNotAbsolute {
        /// The non-absolute path.
        target: String,
    },
    /// The filesystem image appears corrupted.
    #[snafu(display("Filesystem corruption detected: {}", reason))]
    FilesystemCorrupted {
        /// Description of the corruption.
        reason: &'static str,
    },
    /// The file or directory name exceeds the maximum length (255 characters).
    #[snafu(display("Name too long ({} chars, max 255): '{}'", length, name))]
    NameTooLong {
        /// The offending name.
        name: String,
        /// Actual length.
        length: usize,
    },
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

/// MBR-specific errors.
#[derive(Debug, Snafu)]
pub enum MbrError {
    /// The partition at the given index is not a FAT32 partition.
    #[snafu(display("Not a fat32 partition: {index}"))]
    InvalidPartition {
        /// Partition table index.
        index: usize,
    },
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
        match err {
            binrw::Error::Io(_err) => Self::from(IoError::other("IoError")),
            _ => Self::from(IoError::other("binrw error")),
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
