use crate::ClusterId;
use crate::PathBuf;
use crate::api::raw_directory_entry::Attributes;
use crate::api::timestamp::VfatTimestamp;
use alloc::string::String;

/// Metadatas are common to every entry type.
#[derive(Debug, Clone)]
pub struct Metadata {
    creation: VfatTimestamp,
    last_update: VfatTimestamp,
    //last_access: VfatTimestamp,
    pub(crate) name: String,
    /// Size of this file in bytes. For directories, it should be the sum of the sizes
    /// occupied by the metadatas of the contained files.
    pub(crate) size: u32,
    /// The path to this file - it does include the file name.
    path: PathBuf,
    /// empty files with size 0 should have first Cluster == 0.
    pub(crate) cluster: ClusterId,
    /// The path to this file - it doesn't include the file name.
    parent: PathBuf,
    pub(crate) attributes: Attributes,
}

impl Metadata {
    pub(crate) fn new<S: AsRef<str>>(
        creation: VfatTimestamp,
        last_update: VfatTimestamp,
        //last_access: VfatTimestamp,
        name: S,
        size: u32,
        path: PathBuf,
        cluster: ClusterId,
        parent: PathBuf,
        attributes: Attributes,
    ) -> Self {
        Self {
            creation,
            last_update,
            //last_access,
            name: String::from(name.as_ref()),
            size,
            path,
            cluster,
            parent,
            attributes,
        }
    }
}
impl Metadata {
    /// Returns the file size in bytes.
    pub fn size(&self) -> usize {
        self.size as usize
    }

    /*
    fn last_access(&self) -> Option<VfatTimestamp> {
        //Some(self.last_access)
        None
    }
    */

    pub(crate) fn last_update(&self) -> Option<VfatTimestamp> {
        Some(self.last_update)
    }
    // TODO: why are these optional?
    pub(crate) fn creation(&self) -> Option<VfatTimestamp> {
        Some(self.creation)
    }

    /// Returns the full path to this entry (including the entry name).
    pub fn full_path(&self) -> &PathBuf {
        &self.path
    }
    pub(crate) fn parent(&self) -> &PathBuf {
        &self.parent
    }
    /// Returns the entry's name (file or directory name, without path).
    pub fn name(&self) -> &str {
        &self.name
    }
    pub(crate) fn has_no_cluster_allocated(&self) -> bool {
        self.cluster == ClusterId::new(0)
    }
}
