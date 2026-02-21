use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::mem;

use log::{debug, info};
use snafu::ensure;

use crate::api::raw_directory_entry::EntryId::Deleted;
use crate::api::raw_directory_entry::{
    unknown_entry_convert_to_bytes_2, Attributes, LongFileNameEntry, RegularDirectoryEntry,
    UnknownDirectoryEntry, VfatDirectoryEntry,
};
use crate::api::{DirectoryEntry, File, Metadata, VfatMetadataTrait};
use crate::cluster::cluster_reader::ClusterChainReader;
use crate::{error, PathBuf};
use crate::{ClusterId, VfatFS};

// TODO: this assumes sector size
const SECTOR_SIZE: usize = 512;
const ENTRIES_AMOUNT: usize = SECTOR_SIZE / size_of::<UnknownDirectoryEntry>();
const BUF_SIZE: usize = size_of::<UnknownDirectoryEntry>() * ENTRIES_AMOUNT;

/// Converts a raw byte buffer into an array of [`UnknownDirectoryEntry`] values.
pub fn unknown_entry_convert_from_bytes_entries(
    entries: [u8; BUF_SIZE],
) -> [UnknownDirectoryEntry; ENTRIES_AMOUNT] {
    // Initialize result array using const fn from_fn for Copy types
    let mut result: [UnknownDirectoryEntry; ENTRIES_AMOUNT] =
        [UnknownDirectoryEntry::from([0u8; 32]); ENTRIES_AMOUNT];

    // Parse each 32-byte chunk into an UnknownDirectoryEntry
    for (i, chunk) in entries
        .chunks_exact(size_of::<UnknownDirectoryEntry>())
        .enumerate()
    {
        let entry_bytes: [u8; 32] = chunk.try_into().expect("chunk size mismatch");
        result[i] = UnknownDirectoryEntry::from(entry_bytes);
    }
    result
}

/// The kind of entry to create inside a directory.
#[derive(Debug)]
pub enum EntryType {
    /// A regular file.
    File,
    /// A subdirectory.
    Directory,
    // Link
}

/// A directory is composed of "DirectoryEntry" elements.
/// Every directory has at least two pseudo directories: "." (current directory) and ".." (parent directory)
///
#[derive(Debug)]
pub struct Directory {
    pub(crate) vfat_filesystem: VfatFS,
    /// Metadata for this directory (name, path, cluster, timestamps, etc.).
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
        let lock = self.vfat_filesystem.fs_lock.clone();
        let _guard = lock.read();
        self.contains_unlocked(name)
    }

    pub(crate) fn contains_unlocked(&self, name: &str) -> error::Result<bool> {
        for entry in self.contents_unlocked()? {
            if entry.name() == name {
                return Ok(true);
            }
        }
        Ok(false)
    }
    /// Create a new file in this directory
    ///
    pub fn create_file(&mut self, name: String) -> error::Result<File> {
        let lock = self.vfat_filesystem.fs_lock.clone();
        let _guard = lock.write();
        Ok(self.create(name, EntryType::File)?.into_file_unchecked())
    }

    /// Create a new directory in this directory
    ///
    pub fn create_directory(&mut self, name: String) -> error::Result<Directory> {
        let lock = self.vfat_filesystem.fs_lock.clone();
        let _guard = lock.write();
        Ok(self
            .create(name, EntryType::Directory)?
            .into_directory_unchecked())
    }

    /// Used to create a new entry in this directory
    fn create(&mut self, name: String, entry_type: EntryType) -> error::Result<DirectoryEntry> {
        info!(
            "Creating {:?} entry with name '{:?}' in directory '{:?}'",
            entry_type,
            name,
            self.metadata.name()
        );
        if self.contains_unlocked(&name)? {
            return Err(error::VfatRsError::NameAlreadyInUse { target: name });
        }

        // 1. Create metadata:
        let metadata = self.create_metadata_for_new_entry(name.as_str(), &entry_type)?;

        // 2. Based on the name, create one or more LFN and the Regular entry.
        let existing_short_names = self.collect_short_names()?;
        let entries: Vec<UnknownDirectoryEntry> = VfatDirectoryEntry::new_vfat_entry(
            name.as_str(),
            metadata.cluster,
            Self::attributes_from_entry(&entry_type),
            &existing_short_names,
        );
        let entries_len = entries.len();
        let first_empty_spot_offset = if let Some(spot) = self.last_entry_spot {
            spot
        } else {
            self.find_first_empty_spot_offset(entries_len)?
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
            let entry: [u8; size_of::<UnknownDirectoryEntry>()] = unknown_entry.into();
            ccw.write(&entry)?;
        }

        if let EntryType::Directory = entry_type {
            let entries =
                VfatDirectoryEntry::create_pseudo_dir_entries(metadata.cluster, ClusterId::new(0));
            let mut cw = self.vfat_filesystem.cluster_chain_writer(metadata.cluster);
            let buf = unknown_entry_convert_to_bytes_2(entries);
            cw.write(&buf)?;
        }
        // Invalidate cached spot so next operation re-scans for deleted entries
        self.last_entry_spot = None;

        Ok(match entry_type {
            EntryType::Directory => {
                DirectoryEntry::new_directory(metadata, self.vfat_filesystem.clone())
            }
            EntryType::File => DirectoryEntry::new_file(metadata, self.vfat_filesystem.clone()),
        })
    }

    /// Searches for a contiguous run of reusable slots (deleted 0xE5 or end-of-entries 0x00)
    /// that can fit `slots_needed` entries. Returns the byte offset of the first slot in
    /// the run. Falls back to end-of-entries / end-of-cluster if no deleted run is large enough.
    fn find_first_empty_spot_offset(&self, slots_needed: usize) -> error::Result<usize> {
        let mut cluster_chain_reader = self.cluster_chain_reader();
        let mut buff = [0u8; BUF_SIZE];
        let mut offset = 0;
        // Track the start and length of the current contiguous reusable run
        let mut run_start: Option<usize> = None;
        let mut run_len: usize = 0;

        while cluster_chain_reader.read(&mut buff)? > 0 {
            let unknown_entries: [UnknownDirectoryEntry; ENTRIES_AMOUNT] =
                unknown_entry_convert_from_bytes_entries(buff);
            for entry in unknown_entries.iter() {
                if entry.is_end_of_entries() {
                    // End-of-entries: any accumulated run (or this position) works
                    if let Some(start) = run_start {
                        if run_len >= slots_needed {
                            return Ok(start);
                        }
                    }
                    // Fall back to end-of-entries position (original behavior)
                    return Ok(offset);
                } else if entry.is_deleted() {
                    if run_start.is_none() {
                        run_start = Some(offset);
                        run_len = 0;
                    }
                    run_len += 1;
                    if run_len >= slots_needed {
                        return Ok(run_start.unwrap());
                    }
                } else {
                    // Live entry — reset the run
                    run_start = None;
                    run_len = 0;
                }
                offset += size_of::<UnknownDirectoryEntry>();
            }
            buff = [0u8; BUF_SIZE];
        }
        // Navigated the full cluster chain — append at the end
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
        debug!("Going to use as cluster id: {}", cluster_id);
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

    /// Returns an entry from inside this directory.
    fn get_entry(&mut self, target_filename: &str) -> error::Result<DirectoryEntry> {
        self.contents_unlocked()?
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

    /// Delete the entry named `target_name` from this directory.
    pub fn delete(&mut self, target_name: String) -> error::Result<()> {
        let lock = self.vfat_filesystem.fs_lock.clone();
        let _guard = lock.write();
        self.delete_unlocked(target_name)
    }

    fn delete_unlocked(&mut self, target_name: String) -> error::Result<()> {
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
            let contents = directory.contents_unlocked()?;
            if contents.len() > PSEUDO_FOLDERS.len() {
                return Err(error::VfatRsError::NonEmptyDirectory {
                    target: directory.metadata.name().to_string(),
                    contents: contents
                        .into_iter()
                        .map(|entry| entry.name().to_string())
                        .filter(|entry_name| !PSEUDO_FOLDERS.contains(&entry_name.as_str()))
                        .collect::<Vec<_>>()
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

    /// Find a named entry in this directory, returning its index and associated LFN entries.
    fn find_entry_index(
        &self,
        target_name: &str,
    ) -> error::Result<(usize, RegularDirectoryEntry, Vec<LongFileNameEntry>)> {
        let entries = self.contents_direntry()?;
        let mut lfn_name_buff: Vec<(u8, String)> = Vec::new();
        let mut lfn_entries_buff: Vec<LongFileNameEntry> = Vec::new();

        for (index, dir_entry) in entries.into_iter().enumerate() {
            match dir_entry {
                VfatDirectoryEntry::LongFileName(lfn) => {
                    lfn_name_buff.push((lfn.sequence_number.get_position(), lfn.collect_name()));
                    lfn_entries_buff.push(lfn);
                }
                VfatDirectoryEntry::Deleted(_) => {
                    lfn_name_buff.clear();
                    lfn_entries_buff.clear();
                }
                VfatDirectoryEntry::Regular(regular) => {
                    let name = if !lfn_name_buff.is_empty() {
                        Self::string_from_lfn(mem::take(&mut lfn_name_buff))
                    } else {
                        regular.full_name()
                    };
                    if name == target_name {
                        return Ok((index, regular, mem::take(&mut lfn_entries_buff)));
                    }
                    lfn_entries_buff.clear();
                }
                VfatDirectoryEntry::EndOfEntries(_) => break,
            }
        }
        Err(error::VfatRsError::FileNotFound {
            target: target_name.to_string(),
        })
    }

    fn delete_entry(&mut self, target_name: String) -> error::Result<()> {
        info!("Running delete entry");
        let (index, regular, lfn_entries) = self.find_entry_index(&target_name)?;

        // set all the lfn entries as deleted.
        for (i, lfn) in lfn_entries.into_iter().rev().enumerate() {
            let mut unknown: UnknownDirectoryEntry = lfn.into();
            unknown.set_id(Deleted);
            self.update_entry_by_index(unknown, index - i - 1)?;
        }
        let mut unknown: UnknownDirectoryEntry = regular.into();
        unknown.set_id(Deleted);
        self.update_entry_by_index(unknown, index)?;
        Ok(())
    }
    fn delete_cluster_chain(&mut self, entry: &DirectoryEntry) -> error::Result<()> {
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
            entries.extend(
                unknown_entries
                    .iter()
                    .map(VfatDirectoryEntry::from)
                    .filter(filter_invalid),
            );
        }
        Ok(entries)
    }

    /// Collect the 8.3 short names of all regular entries in this directory.
    fn collect_short_names(&self) -> error::Result<Vec<[u8; 8]>> {
        Ok(self
            .contents_direntry()?
            .into_iter()
            .filter_map(|e| {
                if let VfatDirectoryEntry::Regular(r) = e {
                    Some(r.file_name)
                } else {
                    None
                }
            })
            .collect())
    }

    /// Returns the total number of raw directory entry slots in use (regular,
    /// LFN, and deleted — everything except end-of-entries markers).
    /// Useful for verifying that deleted slots are being reclaimed.
    pub fn raw_entry_count(&self) -> error::Result<usize> {
        Ok(self.contents_direntry()?.len())
    }

    /// Returns all entries (files and subdirectories) contained in this directory.
    pub fn contents(&self) -> error::Result<Vec<DirectoryEntry>> {
        let lock = self.vfat_filesystem.fs_lock.clone();
        let _guard = lock.read();
        self.contents_unlocked()
    }

    pub(crate) fn contents_unlocked(&self) -> error::Result<Vec<DirectoryEntry>> {
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
                        Self::string_from_lfn(mem::take(&mut lfn_buff))
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
                        DirectoryEntry::new_directory
                    } else {
                        DirectoryEntry::new_file
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
            .map(|(_, s)| s)
            .fold(String::new(), |mut acc, s| {
                acc.push_str(&s);
                acc
            })
    }

    /// Rename or move `target_name` to `destination_path`.
    pub fn rename(
        &mut self,
        target_name: String,
        destination_path: crate::PathBuf,
    ) -> error::Result<()> {
        let lock = self.vfat_filesystem.fs_lock.clone();
        let _guard = lock.write();
        self.rename_unlocked(target_name, destination_path)
    }

    fn rename_unlocked(
        &mut self,
        target_name: String,
        destination_path: crate::PathBuf,
    ) -> error::Result<()> {
        let dest_str = destination_path.display().to_string();
        let dest_trimmed = dest_str.trim_end_matches('/');

        // Extract new name (last path component) and parent directory path
        let (dest_parent_str, new_name) = match dest_trimmed.rfind('/') {
            Some(0) => ("/".to_string(), dest_trimmed[1..].to_string()),
            Some(pos) => (
                dest_trimmed[..pos].to_string(),
                dest_trimmed[pos + 1..].to_string(),
            ),
            None => {
                return Err(error::VfatRsError::FileNotFound { target: dest_str });
            }
        };
        if new_name.is_empty() {
            return Err(error::VfatRsError::FileNotFound { target: dest_str });
        }
        let dest_parent: crate::PathBuf = dest_parent_str.as_str().into();

        let target_entry = self.get_entry(&target_name)?;
        let mut metadata = target_entry.metadata;

        // Determine if this is a same-directory rename or cross-directory move
        let source_parent_path = self.metadata.full_path();
        if dest_parent == *source_parent_path {
            // Same directory: use existing in-place rename
            return self.inner_rename(target_name, new_name, &mut metadata);
        }

        // Cross-directory move
        // If moving a directory, guard against circular moves
        if metadata.attributes.is_directory() {
            let parent_str = source_parent_path.display().to_string();
            let sep = if parent_str.ends_with('/') { "" } else { "/" };
            let source_entry_path: crate::PathBuf =
                alloc::format!("{}{}{}", parent_str, sep, target_name).into();
            if destination_path.starts_with(&source_entry_path) {
                return Err(error::VfatRsError::CircularMove {
                    source_path: source_entry_path.display().to_string(),
                    destination_path: dest_str,
                });
            }
        }

        // Resolve destination directory
        let dest_dir_entry = self
            .vfat_filesystem
            .get_from_absolute_path_unlocked(dest_parent.clone())?;
        let mut dest_dir = dest_dir_entry.into_directory_or_not_found()?;

        // POSIX semantics: if destination name already exists, delete it
        if dest_dir.contains_unlocked(&new_name)? {
            dest_dir.delete_unlocked(new_name.clone())?;
        }

        // Write new entries in the destination directory
        let attributes = metadata.attributes;
        let existing_short_names = dest_dir.collect_short_names()?;
        let entries: Vec<UnknownDirectoryEntry> =
            VfatDirectoryEntry::new_vfat_entry(new_name.as_str(), metadata.cluster, attributes, &existing_short_names);
        let entries_len = entries.len();
        let first_empty_spot_offset = if let Some(spot) = dest_dir.last_entry_spot {
            spot
        } else {
            dest_dir.find_first_empty_spot_offset(entries_len)?
        };
        let mut ccw = dest_dir
            .vfat_filesystem
            .cluster_chain_writer(dest_dir.metadata.cluster);
        ccw.seek(first_empty_spot_offset)?;
        for unknown_entry in entries.into_iter() {
            let entry: [u8; size_of::<UnknownDirectoryEntry>()] = unknown_entry.into();
            ccw.write(&entry)?;
        }
        dest_dir.last_entry_spot = None;

        // Delete old entries from source directory
        self.delete_entry(target_name)?;

        // For directory moves, update the ".." pseudo-entry to point to new parent
        if metadata.attributes.is_directory() && !metadata.has_no_cluster_allocated() {
            Self::update_dotdot_cluster(
                &self.vfat_filesystem,
                metadata.cluster,
                dest_dir.metadata.cluster,
            )?;
        }

        metadata.name = new_name;
        Ok(())
    }

    /// Update the ".." pseudo-entry inside a directory to point to a new parent cluster.
    fn update_dotdot_cluster(
        vfat: &VfatFS,
        dir_cluster: ClusterId,
        new_parent_cluster: ClusterId,
    ) -> error::Result<()> {
        // ".." is always the second entry (index 1)
        let dotdot_index = 1;
        let index_offset = size_of::<UnknownDirectoryEntry>() * dotdot_index;

        // Read the existing ".." entry
        let mut buf = [0u8; size_of::<UnknownDirectoryEntry>()];
        let mut reader = vfat.cluster_chain_reader(dir_cluster);
        // Skip the "." entry
        let mut skip_buf = [0u8; size_of::<UnknownDirectoryEntry>()];
        reader.read(&mut skip_buf)?;
        reader.read(&mut buf)?;

        let unknown = UnknownDirectoryEntry::from(buf);
        let mut regular: RegularDirectoryEntry = unknown.into();

        // Update cluster to new parent (root cluster 2 maps to 0 per FAT convention)
        let new_parent = if new_parent_cluster == ClusterId::new(2) {
            ClusterId::new(0)
        } else {
            new_parent_cluster
        };
        let (high, low) = new_parent.into_high_low();
        regular.high_16bits = high;
        regular.low_16bits = low;

        // Write it back
        let updated: UnknownDirectoryEntry = regular.into();
        let write_buf: [u8; size_of::<UnknownDirectoryEntry>()] = updated.into();
        let mut writer = vfat.cluster_chain_writer(dir_cluster);
        writer.seek(index_offset)?;
        writer.write(&write_buf)?;
        Ok(())
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
        let existing_short_names = self.collect_short_names()?;
        let entries: Vec<UnknownDirectoryEntry> =
            VfatDirectoryEntry::new_vfat_entry(new_name.as_str(), metadata.cluster, attributes, &existing_short_names);
        let entries_len = entries.len();
        let first_empty_spot_offset = if let Some(spot) = self.last_entry_spot {
            spot
        } else {
            self.find_first_empty_spot_offset(entries_len)?
        };
        let mut ccw = self
            .vfat_filesystem
            .cluster_chain_writer(self.metadata.cluster);
        ccw.seek(first_empty_spot_offset)?;

        for unknown_entry in entries.into_iter() {
            let entry: [u8; size_of::<UnknownDirectoryEntry>()] = unknown_entry.into();
            ccw.write(&entry)?;
        }
        metadata.name = new_name;

        // Invalidate cached spot so next operation re-scans for deleted entries
        self.last_entry_spot = None;
        self.delete_entry(target_name)?;
        Ok(())
    }

    // TODO: Currently this doesn't support renaming file, just updating/replacing metadatas...
    fn update_entry_inner(
        &mut self,
        target_name: String,
        new_entry: UnknownDirectoryEntry,
    ) -> error::Result<()> {
        debug!("Running update entry routine...");
        let (index, _, _) = self.find_entry_index(&target_name)?;
        self.update_entry_by_index(new_entry, index)
    }

    // Replace entry with index `index` with input `entry`.
    // TODO: when reading the file, keep the index around to avoid scanning to locate the file again.
    pub(crate) fn update_entry_by_index(
        &self,
        entry: UnknownDirectoryEntry,
        index: usize,
    ) -> error::Result<()> {
        let index_offset = size_of::<UnknownDirectoryEntry>() * index;
        let buf: [u8; size_of::<UnknownDirectoryEntry>()] = entry.into();

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

impl VfatMetadataTrait for Directory {
    fn metadata(&self) -> &Metadata {
        &self.metadata
    }
}

#[cfg(test)]
mod test {
    extern crate std;

    use crate::api::raw_directory_entry::EntryId;

    #[test]
    fn valid_entry_id() {
        let id: u8 = 0x10;
        assert!(matches!(EntryId::from(id), EntryId::Valid(_)));
        //id
    }
}
