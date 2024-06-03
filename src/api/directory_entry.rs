use crate::api::timestamp::VfatTimestamp;
use crate::api::{Directory, File, Metadata};
use crate::{Result, VfatFS};

/// This is a library's user interface. Each directory can contain either a File or a Directory.
#[derive(Debug)]
enum EntryKind {
    File,
    Directory,
}
pub trait VfatMetadataTrait {
    fn metadata(&self) -> &Metadata;
    fn name(&self) -> &str {
        self.metadata().name()
    }
    fn creation(&self) -> VfatTimestamp {
        self.metadata().creation().unwrap()
    }
}
impl VfatMetadataTrait for DirectoryEntry {
    fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

/// A directory entry: either a file or a directory.
#[derive(Debug)]
pub struct DirectoryEntry {
    kind: EntryKind,
    pub metadata: Metadata,
    vfat_filesystem: VfatFS,
}
impl DirectoryEntry {
    pub fn new_file(metadata: Metadata, vfat_filesystem: VfatFS) -> Self {
        Self {
            kind: EntryKind::File,
            metadata,
            vfat_filesystem,
        }
    }
    pub fn new_directory(metadata: Metadata, vfat_filesystem: VfatFS) -> Self {
        Self {
            kind: EntryKind::Directory,
            metadata,
            vfat_filesystem,
        }
    }
}

impl DirectoryEntry {
    pub(crate) fn is_dir(&self) -> bool {
        matches!(&self.kind, EntryKind::Directory)
    }

    pub fn into_directory(self) -> Option<Directory> {
        self.is_dir()
            .then(|| Directory::new(self.vfat_filesystem, self.metadata))
    }
    pub fn into_directory_unchecked(self) -> Directory {
        Directory::new(self.vfat_filesystem, self.metadata)
    }
    pub fn into_directory_or_not_found(self) -> Result<Directory> {
        if self.is_dir() {
            Ok(self.into_directory_unchecked())
        } else {
            Err(crate::error::VfatRsError::EntryNotFound {
                target: self.metadata.name().into(),
            })
        }
    }
    fn is_file(&self) -> bool {
        !self.is_dir()
    }
    pub fn into_file(self) -> Option<File> {
        self.is_file()
            .then(|| File::new(self.vfat_filesystem, self.metadata))
    }
    pub fn into_file_unchecked(self) -> File {
        self.is_file()
            .then(|| File::new(self.vfat_filesystem, self.metadata))
            .unwrap()
    }
}

impl From<Directory> for DirectoryEntry {
    fn from(directory: Directory) -> Self {
        DirectoryEntry::new_directory(directory.metadata, directory.vfat_filesystem)
    }
}
