use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::{IntoIter, Vec};
use core::mem;

use log::{debug, error, info};
use snafu::ensure;

use crate::api::directory_entry::EntryId::Deleted;
use crate::api::directory_entry::{
    unknown_entry_convert_to_bytes_2, Attributes, LongFileNameEntry, RegularDirectoryEntry,
    UnknownDirectoryEntry, VfatDirectoryEntry,
};
use crate::api::{File, Metadata, VfatEntry};
use crate::cluster::cluster_reader::ClusterChainReader;
use crate::{error, PathBuf};
use crate::{ClusterId, VfatFS, VfatMetadataTrait};

// TODO: this assumes sector size
const SECTOR_SIZE: usize = 512;
const ENTRIES_AMOUNT: usize = SECTOR_SIZE / mem::size_of::<UnknownDirectoryEntry>();
const BUF_SIZE: usize = mem::size_of::<UnknownDirectoryEntry>() * ENTRIES_AMOUNT;

pub fn unknown_entry_convert_from_bytes_entries(
    entries: [u8; BUF_SIZE],
) -> [UnknownDirectoryEntry; ENTRIES_AMOUNT] {
    unsafe { mem::transmute(entries) }
}

pub enum EntryType {
    File,
    Directory,
    // Link
}

/// This is the public interface to the directory concept.
/// A directory is composed of "DirectoryEntry" elements.
/// The directory supports Long File Name (LFN)
/// Externally, DirectoryEntry are converted to directory or files. Deleted elements are hidden.
/// Every directory has two pseudo directories: "." (current directory) and ".." (parent directory)
#[derive(Debug)]
pub struct Directory {
    pub(crate) vfat_filesystem: VfatFS,
    pub metadata: Metadata,
    // An optimization, if we already created an entry, we know the offset of the last position.
    last_entry_spot: Option<usize>,
}

impl Directory {
    pub(crate) fn new(vfat_filesystem: VfatFS, metadata: Metadata) -> Self {
        Self {
            vfat_filesystem,
            metadata,
            last_entry_spot: None,
        }
    }

    /// Returns true if an entry called "name" is contained in this directory
    ///
    pub fn contains(&self, name: &str) -> error::Result<bool> {
        for entry in self.contents()? {
            if entry.name() == name {
                return Ok(true);
            }
        }
        Ok(false)
    }
    /// Create a new file in this directory
    ///
    pub fn create_file(&mut self, name: String) -> error::Result<File> {
        Ok(self.create(name, EntryType::File)?.into_file_unchecked())
    }

    /// Create a new directory in this directory
    ///
    pub fn create_directory(&mut self, name: String) -> error::Result<Directory> {
        Ok(self
            .create(name, EntryType::Directory)?
            .into_directory_unchecked())
    }

    /// Used to create a new entry in this directory
    fn create(&mut self, name: String, entry_type: EntryType) -> error::Result<VfatEntry> {
        if self.contains(&name)? {
            return Err(error::VfatRsError::NameAlreadyInUse { target: name });
        }

        // 1. Create metadata:
        let metadata = self.create_metadata_for_new_entry(name.as_str(), &entry_type)?;

        // 2. Based on the name, create one or more LFN and the Regular entry.
        let entries: Vec<UnknownDirectoryEntry> = VfatDirectoryEntry::new_vfat_entry(
            name.as_str(),
            metadata.cluster,
            Self::attributes_from_entry(&entry_type),
        );
        let entries_len = entries.len();
        let first_empty_spot_offset = if self.last_entry_spot.is_none() {
            self.find_first_empty_spot_offset()?
        } else {
            self.last_entry_spot.unwrap()
        };

        info!(
            "Going to use as metadata: {:?}. self metadatapath= '{}', selfmetadata name = '{}'. My attributes: {:?}, cluster: {:?}",
            metadata,
            self.metadata.full_path().display(),
            self.metadata.name(),
            self.metadata.attributes,
            self.metadata.cluster
        );
        info!(
            "Found spot: {:?}, Going to append entries: {:?}",
            first_empty_spot_offset, entries
        );

        let mut ccw = self
            .vfat_filesystem
            .cluster_chain_writer(self.metadata.cluster);
        ccw.seek(first_empty_spot_offset)?;

        for unknown_entry in entries.into_iter() {
            let entry: [u8; mem::size_of::<UnknownDirectoryEntry>()] = unknown_entry.into();
            ccw.write(&entry)?;
        }

        if let EntryType::Directory = entry_type {
            let entries =
                VfatDirectoryEntry::create_pseudo_dir_entries(metadata.cluster, ClusterId::new(0));
            let mut cw = self.vfat_filesystem.cluster_chain_writer(metadata.cluster);
            let buf = unknown_entry_convert_to_bytes_2(entries);
            cw.write(&buf)?;
        }
        // finally, update entries:
        self.last_entry_spot =
            Some(first_empty_spot_offset + entries_len * mem::size_of::<UnknownDirectoryEntry>());

        Ok(match entry_type {
            EntryType::Directory => {
                VfatEntry::new_directory(metadata, self.vfat_filesystem.clone())
            }
            EntryType::File => VfatEntry::new_file(metadata, self.vfat_filesystem.clone()),
        })
    }

    /// Searches for `spots_needed` in all the clusters allocated to this directory
    /// it only searches for empty spots, it won't allow (for now? TODO) replacing deleted entries.
    ///
    /// Returns:
    ///  usize = index of the first EndOfEntries
    ///  bool = enough spots found, no allocation needed.
    /// TODO: revert commit ddf3fb48a03abff9e8096db72b571efab8307263 or add suppport for reusing deleted entries.
    /// that commit partially implemented this behavior.
    fn find_first_empty_spot_offset(&self) -> error::Result<usize> {
        let mut cluster_chain_reader = self.cluster_chain_reader();
        let mut buff = [0u8; BUF_SIZE];
        let mut offset = 0;
        while cluster_chain_reader.read(&mut buff)? > 0 {
            let unknown_entries: [UnknownDirectoryEntry; ENTRIES_AMOUNT] =
                unknown_entry_convert_from_bytes_entries(buff);
            for entry in unknown_entries.iter() {
                if entry.is_end_of_entries() {
                    return Ok(offset);
                }
                offset += mem::size_of::<UnknownDirectoryEntry>();
            }
            buff = [0u8; BUF_SIZE];
        }
        // we navigated the full cluster, but it's fully used.
        Ok(offset)
    }

    fn create_metadata_for_new_entry(
        &mut self,
        entry_name: &str,
        entry_type: &EntryType,
    ) -> error::Result<Metadata> {
        let path = PathBuf::from(format!(
            "{}{}",
            self.metadata.full_path().display(),
            entry_name
        ));
        let attributes = Self::attributes_from_entry(entry_type);
        let cluster_id = match entry_type {
            // No need to allocate a new cluster
            EntryType::File => ClusterId::new(0),
            // Allocate for directory
            EntryType::Directory => self.vfat_filesystem.allocate_cluster_new_entry()?,
        };
        info!("Going to use as cluster id: {}", cluster_id);
        let size = 0;
        let metadata = Metadata::new(
            self.vfat_filesystem
                .time_manager
                .get_current_vfat_timestamp(),
            self.vfat_filesystem
                .time_manager
                .get_current_vfat_timestamp(),
            entry_name,
            size,
            path,
            cluster_id,
            self.metadata.full_path().clone(),
            attributes,
        );
        Ok(metadata)
    }

    pub(crate) fn iter(&self) -> error::Result<IntoIter<VfatEntry>> {
        self.contents().map(|v| v.into_iter())
    }

    /// Returns an entry from inside this directory.
    fn get_entry(&mut self, target_filename: &str) -> error::Result<VfatEntry> {
        self.contents()?
            .into_iter()
            .find(|name| {
                debug!(
                    "Checking name: {} == {}",
                    name.metadata.name(),
                    target_filename
                );
                name.metadata.name() == target_filename
            })
            .ok_or_else(|| error::VfatRsError::FileNotFound {
                target: target_filename.to_string(),
            })
    }

    pub fn delete(&mut self, target_name: String) -> error::Result<()> {
        info!("Starting delete routine for entry: '{}'. ", target_name);

        const PSEUDO_CURRENT_FOLDER: &str = ".";
        const PSEUDO_PARENT_FOLDER: &str = "..";
        const PSEUDO_FOLDERS: &[&str; 2] = &[PSEUDO_PARENT_FOLDER, PSEUDO_CURRENT_FOLDER];

        ensure!(
            !PSEUDO_FOLDERS.contains(&target_name.as_str()),
            error::CannotDeletePseudoDirSnafu {
                target: target_name,
            }
        );

        let mut target_entry = self.get_entry(&target_name)?;

        if target_entry.is_dir() {
            let directory = target_entry.into_directory_unchecked();
            let contents = directory.contents()?;
            if contents.len() > PSEUDO_FOLDERS.len() {
                return Err(error::VfatRsError::NonEmptyDirectory {
                    target: directory.metadata.name().to_string(),
                    contents: contents
                        .into_iter()
                        .map(|entry| entry.name().to_string())
                        .filter(|entry_name| !PSEUDO_FOLDERS.contains(&entry_name.as_str()))
                        .collect::<Vec<String>>()
                        .join(", "),
                });
            }
            target_entry = directory.into();
        }
        info!("Found target entry: {:?}", target_entry);

        self.delete_cluster_chain(&target_entry)?;
        self.delete_entry(target_name)?;
        Ok(())
    }
    fn delete_entry(&mut self, target_name: String) -> error::Result<()> {
        // to delete the directory entry
        // recreate the entry with lfn, and set deleted id.
        info!("Running delete entry");
        let mut lfn_name_buff: Vec<(u8, String)> = Vec::new();
        let mut lfn_entries_buff: Vec<LongFileNameEntry> = Vec::new();

        let entries = self.contents_direntry()?;

        for (index, dir_entry) in entries.into_iter().enumerate() {
            match dir_entry {
                VfatDirectoryEntry::LongFileName(lfn) => {
                    lfn_name_buff.push((lfn.sequence_number.get_position(), lfn.collect_name()));
                    lfn_entries_buff.push(lfn);
                }
                VfatDirectoryEntry::Deleted(_) => {
                    lfn_entries_buff.clear();
                    lfn_name_buff.clear();
                }
                VfatDirectoryEntry::Regular(regular) => {
                    let name = if !lfn_name_buff.is_empty() {
                        // sort lfn_name_buff by first element of the tuple
                        lfn_name_buff.sort();
                        Self::string_from_lfn(lfn_name_buff)
                    } else {
                        regular.full_name()
                    };
                    if name == target_name {
                        for entry in lfn_entries_buff.into_iter().rev().enumerate() {
                            let mut unknown: UnknownDirectoryEntry = entry.1.into();
                            unknown.set_id(Deleted);
                            self.update_entry_by_index(unknown, index - entry.0 - 1)?;
                        }
                        let mut unknown: UnknownDirectoryEntry = regular.into();
                        unknown.set_id(Deleted);
                        self.update_entry_by_index(unknown, index)?;
                        return Ok(());
                    }
                    lfn_name_buff = Vec::new();
                    lfn_entries_buff = Vec::new();
                }
                // The for loop stops on EndOfEntries
                VfatDirectoryEntry::EndOfEntries(_) => {
                    panic!("This cannot happen! Found EndOfEntries")
                }
            };
        }
        error!("Directory update entry {}: file not found!!", target_name);
        Err(error::VfatRsError::FileNotFound {
            target: target_name,
        })
    }
    fn delete_cluster_chain(&mut self, entry: &VfatEntry) -> error::Result<()> {
        info!(
            "Deleting entry's associated clusters starting at {:?}",
            entry.metadata.cluster
        );
        self.vfat_filesystem
            .delete_fat_cluster_chain(entry.metadata.cluster)
    }

    fn contents_direntry(&self) -> error::Result<Vec<VfatDirectoryEntry>> {
        info!("Directory contents, cluster: {:?}", self.metadata.cluster);

        let mut buf = [0; BUF_SIZE];
        let filter_invalid =
            |entry: &VfatDirectoryEntry| !matches!(*entry, VfatDirectoryEntry::EndOfEntries(_));
        let mut cluster_chain_reader = self.cluster_chain_reader();

        let mut entries = Vec::new();
        while cluster_chain_reader.read(&mut buf)? > 0 {
            let unknown_entries: [UnknownDirectoryEntry; ENTRIES_AMOUNT] =
                unknown_entry_convert_from_bytes_entries(buf);
            //debug!("Unknown entries: {:?}", unknown_entries);
            /*#[cfg(debug_assertions)]
            unknown_entries
                .iter()
                .map(VfatDirectoryEntry::from)
                //.take_while(filter_invalid)
                .for_each(|entry| info!("unknown entry to vfat directory entry: {:?}", entry));
            */
            entries.extend(
                unknown_entries
                    .iter()
                    .map(VfatDirectoryEntry::from)
                    .filter(filter_invalid),
            );
        }
        Ok(entries)
    }

    pub fn contents(&self) -> error::Result<Vec<VfatEntry>> {
        info!("Directory contents, cluster: {:?}", self.metadata.cluster);

        let entries = self.contents_direntry()?;
        let mut contents = Vec::new();

        let mut lfn_buff: Vec<(u8, String)> = Vec::new();
        for dir_entry in entries {
            debug!("Found entry: {:?}", dir_entry);
            match dir_entry {
                VfatDirectoryEntry::LongFileName(lfn) => {
                    lfn_buff.push((lfn.sequence_number.get_position(), lfn.collect_name()))
                }
                VfatDirectoryEntry::Deleted(_) => {
                    lfn_buff.clear();
                }
                VfatDirectoryEntry::Regular(regular) => {
                    let name = if !lfn_buff.is_empty() {
                        let ret = Self::string_from_lfn(lfn_buff);
                        lfn_buff = Vec::new();
                        ret
                    } else {
                        regular.full_name()
                    };

                    let path = PathBuf::from(format!(
                        "{}{name}{}",
                        self.metadata.full_path().display(),
                        if regular.is_dir() { "/" } else { "" }
                    ));

                    let metadata = Metadata::new(
                        regular.creation_time,
                        regular.last_modification_time,
                        name,
                        regular.file_size,
                        path,
                        regular.cluster(),
                        self.metadata.full_path().clone(),
                        regular.attributes,
                    );

                    debug!("Metadata: {:?}", metadata);

                    let new_fn = if regular.is_dir() {
                        VfatEntry::new_directory
                    } else {
                        VfatEntry::new_file
                    };

                    contents.push(new_fn(metadata, self.vfat_filesystem.clone()));
                }
                // The for loop stops on EndOfEntries
                VfatDirectoryEntry::EndOfEntries(_) => {
                    panic!("This cannot happen! Found EndOfEntries")
                }
            }
        }
        Ok(contents)
    }

    pub(crate) fn update_entry(&mut self, metadata: Metadata) -> error::Result<()> {
        let target_name = metadata.name().to_string();
        info!("Running update entry on target name: {}", target_name);
        let regular: RegularDirectoryEntry = metadata.into();
        self.update_entry_inner(target_name, regular.into())
    }

    fn cluster_chain_reader(&self) -> ClusterChainReader {
        self.vfat_filesystem
            .cluster_chain_reader(self.metadata.cluster)
    }

    // create a string from a vec
    fn string_from_lfn(mut lfn_vec: Vec<(u8, String)>) -> String {
        // lfn are not assumed to be created in order, hence we need to
        // sort using the sequence number
        lfn_vec.sort();
        // Build the string.
        lfn_vec
            .into_iter()
            .map(|(_, name)| name)
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn rename(&mut self, target_name: String, new_name: String) -> error::Result<()> {
        // TODO: ensure new_name doesn't exists. This is tricky because of TOCTOU
        // it should acquire a lock on the fs. check and create.
        // TODO: try to test file deletion and read after the delete. It looks like it works though it shouldn't.

        let target_entry = self.get_entry(&target_name)?;

        let mut metadata = target_entry.metadata;
        self.inner_rename(target_name, new_name, &mut metadata)
    }
    fn inner_rename(
        &mut self,
        target_name: String,
        new_name: String,
        metadata: &mut Metadata,
    ) -> error::Result<()> {
        // create lfn from existing file
        // delete old file
        let attributes = metadata.attributes;
        let entries: Vec<UnknownDirectoryEntry> =
            VfatDirectoryEntry::new_vfat_entry(new_name.as_str(), metadata.cluster, attributes);
        let entries_len = entries.len();
        let first_empty_spot_offset = if self.last_entry_spot.is_none() {
            self.find_first_empty_spot_offset()?
        } else {
            self.last_entry_spot.unwrap()
        };

        info!(
            "Going to use as metadata: {:?}. self metadatapath= '{}', selfmetadata name = '{}'. My attributes: {:?}, cluster: {:?}",
            metadata,
            self.metadata.full_path().display(),
            self.metadata.name(),
            self.metadata.attributes,
            self.metadata.cluster
        );
        info!(
            "Found spot: {:?}, Going to append entries: {:?}",
            first_empty_spot_offset, entries
        );

        let mut ccw = self
            .vfat_filesystem
            .cluster_chain_writer(self.metadata.cluster);
        ccw.seek(first_empty_spot_offset)?;

        for unknown_entry in entries.into_iter() {
            let entry: [u8; mem::size_of::<UnknownDirectoryEntry>()] = unknown_entry.into();
            ccw.write(&entry)?;
        }
        metadata.name = new_name;

        // finally, update entries:
        self.last_entry_spot =
            Some(first_empty_spot_offset + entries_len * mem::size_of::<UnknownDirectoryEntry>());
        self.delete_entry(target_name)?;
        Ok(())
    }

    // TODO: Currently this doesn't support renaming file, just updating/replacing metadatas...
    // TODO: lfn_name_buff should be sorted by position.
    fn update_entry_inner(
        &mut self,
        target_name: String,
        new_entry: UnknownDirectoryEntry,
    ) -> error::Result<()> {
        debug!("Running update entry routine...");
        let mut lfn_buff: Vec<(u8, String)> = Vec::new();

        let entries = self.contents_direntry()?;
        //info!("target_name: {}, entries: {:?}", target_name, entries);
        for (index, dir_entry) in entries.iter().enumerate() {
            match dir_entry {
                VfatDirectoryEntry::LongFileName(lfn) => {
                    lfn_buff.push((lfn.sequence_number.get_position(), lfn.collect_name()))
                }
                VfatDirectoryEntry::Deleted(_) => lfn_buff.clear(),
                VfatDirectoryEntry::Regular(regular) => {
                    let name = if !lfn_buff.is_empty() {
                        lfn_buff.sort();
                        let file_name = Self::string_from_lfn(lfn_buff);
                        // prepare the buffer for the next file.
                        lfn_buff = Vec::new();
                        file_name
                    } else {
                        regular.full_name()
                    };
                    if name == target_name {
                        self.update_entry_by_index(new_entry, index)?;
                        return Ok(());
                    }
                }
                // The for loop stops on EndOfEntries
                VfatDirectoryEntry::EndOfEntries(_) => {
                    panic!("This cannot happen! Found EndOfEntries")
                }
            };
        }
        error!("Directory update entry {}: file not found!!", target_name);
        Err(error::VfatRsError::FileNotFound {
            target: target_name,
        })
    }

    // Replace entry with index `index` with input `entry`.
    // TODO: when reading the file, keep the index around to avoid scanning to locate the file again.
    pub(crate) fn update_entry_by_index(
        &self,
        entry: UnknownDirectoryEntry,
        index: usize,
    ) -> error::Result<()> {
        let index_offset = mem::size_of::<UnknownDirectoryEntry>() * index;
        let buf: [u8; mem::size_of::<UnknownDirectoryEntry>()] = entry.into();

        let mut ccw = self
            .vfat_filesystem
            .cluster_chain_writer(self.metadata.cluster);
        ccw.seek(index_offset)?;
        ccw.write(&buf)?;
        Ok(())
    }

    fn attributes_from_entry(entry: &EntryType) -> Attributes {
        match entry {
            EntryType::Directory => Attributes::new_directory(),
            EntryType::File => Attributes(0),
        }
    }
}

#[cfg(test)]
mod test {
    extern crate std;

    use crate::api::directory_entry::EntryId;

    #[test]
    fn valid_entry_id() {
        let id: u8 = 0x10;
        assert!(matches!(EntryId::from(id), EntryId::Valid(_)));
        //id
    }
}
