use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::min;
use core::convert::TryInto;
use core::fmt::Debug;
use core::iter;

use log::{debug, info};

use crate::ClusterId;
pub use crate::api::raw_directory_entry::formats::{Attributes, EntryId, attribute};
pub use crate::api::raw_directory_entry::long_file_name_entry::{
    LongFileNameEntry, SequenceNumber,
};
pub use crate::api::raw_directory_entry::regular_entry::RegularDirectoryEntry;
pub use crate::api::raw_directory_entry::unknown_entry::*;
use crate::api::timestamp::VfatTimestamp;

mod formats;
mod long_file_name_entry;
mod regular_entry;
mod unknown_entry;

/// marks previous entry as last in the directory
const ID_LAST_ENTRY_WAS_LAST: u8 = 0x00;

/// marks file as deleted when in name[0]
const ID_DELETED_UNUSED_ENTRY: u8 = 0xE5;

/// The content of a directory on a disk, is a list of entries which can take form of:
///  * A regular entry
///  * A lfn entry.
///  * an EOE - last entry in the chain to signal the end of the directory contents.
/// * a Deleted entry, which might be reused for new entries.
#[derive(Clone, Debug)]
pub(crate) enum VfatDirectoryEntry {
    Regular(RegularDirectoryEntry),
    LongFileName(LongFileNameEntry),
    EndOfEntries(UnknownDirectoryEntry),
    Deleted(UnknownDirectoryEntry),
}

impl From<UnknownDirectoryEntry> for VfatDirectoryEntry {
    fn from(unknown: UnknownDirectoryEntry) -> Self {
        let entry_id = EntryId::from(unknown.id);
        match entry_id {
            EntryId::Deleted => VfatDirectoryEntry::Deleted(unknown),
            EntryId::EndOfEntries => VfatDirectoryEntry::EndOfEntries(unknown),
            EntryId::Valid(_) if unknown.is_lfn() => {
                //info!("Long file name entry: {:?}", long_file_name_entry);
                VfatDirectoryEntry::LongFileName(unknown.into())
            }
            // if this is not an lfn, catchall to a Valid entry:
            EntryId::Valid(_) => VfatDirectoryEntry::Regular(unknown.into()),
        }
    }
}

impl From<&UnknownDirectoryEntry> for VfatDirectoryEntry {
    fn from(unknown: &UnknownDirectoryEntry) -> Self {
        VfatDirectoryEntry::from(*unknown)
    }
}

/// This character can be either 0x00 or 0x20 (space).
/// Because Linux uses 0x20 for Regular filename, I will follow that convention.
const PADDING_CHARACTER: u8 = b' ';
const DOT_CHARACTER: u8 = b'.';

impl VfatDirectoryEntry {
    // pseudo dir entries are entries . and ..
    pub(crate) fn create_pseudo_dir_entries(
        current_dir: ClusterId,
        parent_dir: ClusterId,
    ) -> [UnknownDirectoryEntry; 2] {
        let (current_high, current_low) = current_dir.into_high_low();
        let current_name = [DOT_CHARACTER, 0, 0, 0, 0, 0, 0, 0];
        let file_ext = [PADDING_CHARACTER; 3];
        let attributes = Attributes::new_directory();
        let new_regular_dir_entry = |name, high, low| RegularDirectoryEntry {
            file_name: name,
            file_ext,
            attributes,
            _reseverd_win_nt: 0,
            creation_millis: Default::default(),
            creation_time: VfatTimestamp::new(1385663476),
            last_access_date: 0,
            high_16bits: high,
            last_modification_time: VfatTimestamp::new(1385663476),
            low_16bits: low,
            file_size: 0,
        };
        let current_entry = new_regular_dir_entry(current_name, current_high, current_low);

        let parent_name = [DOT_CHARACTER, DOT_CHARACTER, 0, 0, 0, 0, 0, 0];

        // According to experiments against Linux fat32 driver, when I create a directory under root
        // it uses ClusterId(0) instead of ClusterId(2).
        // I'm not sure the reason behind it, but I need this otherwise the directory is not interpreted
        // correctly.
        let (parent_high, parent_low) = if parent_dir == ClusterId::new(2) {
            ClusterId::new(0)
        } else {
            parent_dir
        }
        .into_high_low();

        let parent_entry = new_regular_dir_entry(parent_name, parent_high, parent_low);
        [
            VfatDirectoryEntry::Regular(current_entry).transmute_into_unknown_dir_entry(),
            VfatDirectoryEntry::Regular(parent_entry).transmute_into_unknown_dir_entry(),
        ]
    }

    /// Convert into a [`RegularDirectoryEntry`] if this is a regular entry.
    pub fn into_regular(self) -> Option<RegularDirectoryEntry> {
        if let Self::Regular(regular_dir_entry) = self {
            Some(regular_dir_entry)
        } else {
            None
        }
    }

    /// Convert any variant into its raw [`UnknownDirectoryEntry`] representation.
    pub fn transmute_into_unknown_dir_entry(self) -> UnknownDirectoryEntry {
        match self {
            Self::Regular(entry) => entry.into(),
            Self::LongFileName(entry) => entry.into(),
            Self::EndOfEntries(entry) | Self::Deleted(entry) => entry,
        }
    }

    #[cfg(test)]
    pub fn into_long_file_name(self) -> Option<LongFileNameEntry> {
        if let Self::LongFileName(long_file_name) = self {
            Some(long_file_name)
        } else {
            None
        }
    }

    /// The short name is derived from the long name as follows:
    /// The extension is the extension of the long name, truncated to
    /// length at most three. The first six bytes of the short name equal
    /// the first six nonspace bytes of the long name, but bytes +,;=[],
    /// that are not allowed under DOS, are replaced by underscore.
    /// Lower case is converted to upper case. The final two (or more, up
    /// to seven, if necessary) bytes become ~1, or, if that exists already,
    /// ~2, etc., up to ~999999.
    // TODO: add some check for presence of another file called in the same way (I'm using always ~1).
    /// Derive the 8.3 short file name from a long name.
    pub fn regular_filename_from(name: &str) -> [u8; 8] {
        Self::regular_filename_from_with_tail(name, 1)
    }

    /// Derive the 8.3 short file name from a long name with a specific numeric tail.
    /// The prefix shrinks as the tail grows: ~1 uses 3-char prefix, ~10 uses 2-char, etc.
    pub fn regular_filename_from_with_tail(name: &str, tail: u32) -> [u8; 8] {
        let replace_invalid_dos_char = |ch| {
            const INVALID_CHARS: [char; 6] = ['+', ',', ';', '=', '[', ']'];
            if INVALID_CHARS.contains(&ch) { '_' } else { ch }
        };

        let tail_str = alloc::format!("~{}", tail);
        let tail_bytes = tail_str.as_bytes();
        let prefix_len = 8 - tail_bytes.len();

        // Use only the base name (before the first dot), strip spaces and dots
        let base_name = name.split('.').next().unwrap_or(name);
        let regular_filename_substr: String = base_name
            .chars()
            .filter(|&ch| ch != ' ' && ch != '.')
            .map(replace_invalid_dos_char)
            .flat_map(char::to_uppercase)
            .take(prefix_len)
            .collect();
        let regular_filename_bytes = regular_filename_substr.as_bytes();
        let mut regular_filename: [u8; 8] = [PADDING_CHARACTER; 8];
        for (i, &b) in regular_filename_bytes.iter().enumerate() {
            regular_filename[i] = b;
        }
        let tail_start = regular_filename_bytes.len();
        for (i, &b) in tail_bytes.iter().enumerate() {
            regular_filename[tail_start + i] = b;
        }
        regular_filename
    }

    /// Returns the extension (if any) with padding characters
    fn get_regular_filename_ext(name: &str) -> [u8; 3] {
        let mut ext: [u8; 3] = [PADDING_CHARACTER; 3];

        if let Some(dot_pos) = name.rfind('.') {
            let ext_str = &name[dot_pos + 1..];
            if (1..=3).contains(&ext_str.len()) && ext_str.chars().all(|c| c.is_ascii_alphabetic())
            {
                for (index, ch) in ext_str.chars().flat_map(|ch| ch.to_uppercase()).enumerate() {
                    ext[index] = ch as u8;
                }
            }
        }
        ext
    }

    /// As seen in: https://www.kernel.org/doc/html/latest/filesystems/vfat.html
    /// The checksum is calculated from the 8.3 name using the following algorithm:
    /// ```c
    /// for (sum = i = 0; i < 11; i++) {
    ///         sum = (((sum&1)<<7)|((sum&0xfe)>>1)) + name[i]
    ///  }
    /// ```
    fn checksum(name: &[u8], ext: &[u8]) -> u8 {
        let mut sum = 0u8;
        for ch in name.iter().chain(ext) {
            sum = ((sum & 1) << 7)
                .wrapping_add((sum & 0xfe) >> 1)
                .wrapping_add(*ch);
        }
        sum
    }
    pub(crate) fn convert<const T: usize>(buf: &[u8]) -> [u16; T] {
        let padding = || iter::repeat(0x0000u16);
        buf.iter()
            .map(|v| *v as u16)
            .chain(padding())
            .take(T)
            .collect::<Vec<u16>>()
            .as_slice()
            .try_into()
            .unwrap()
    }
    // The implementation of this function is inspired by tests I've run on my
    // Linux machine.
    // Even if there is a perfect fit for a RegularFileEntry, Linux creates a new
    // LongFileName nevertheless. I assume the reason for this is because of different
    // char encoding. LFN uses a subset of utf-16 while RegularFileEntry uses ASCII.
    pub(crate) fn new_vfat_entry(
        name: &str,
        cluster_id: ClusterId,
        attributes: Attributes,
        existing_short_names: &[[u8; 8]],
    ) -> crate::error::Result<Vec<UnknownDirectoryEntry>> {
        const MAX_LFN_NAME_LEN: usize = 255;
        if name.len() > MAX_LFN_NAME_LEN {
            return Err(crate::error::VfatRsError::NameTooLong {
                name: String::from(name),
                length: name.len(),
            });
        }

        // Find a non-colliding short name by incrementing the tail number
        let mut tail = 1u32;
        let regular_filename = loop {
            let candidate = Self::regular_filename_from_with_tail(name, tail);
            if !existing_short_names.contains(&candidate) {
                break candidate;
            }
            tail += 1;
        };
        let regular_filename_ext = Self::get_regular_filename_ext(name);
        let checksum = Self::checksum(&regular_filename, &regular_filename_ext);

        info!(
            "regular_filename: {}",
            String::from_utf8_lossy(&regular_filename)
        );
        let (high_cluster_id, low_cluster_id) = cluster_id.into_high_low();

        let regular = RegularDirectoryEntry {
            file_name: regular_filename,
            file_ext: regular_filename_ext,
            attributes,
            _reseverd_win_nt: 0,
            creation_millis: Default::default(),
            creation_time: VfatTimestamp::new(1385663476),
            last_access_date: 0,
            high_16bits: high_cluster_id,
            last_modification_time: VfatTimestamp::new(1385663476),
            low_16bits: low_cluster_id,
            file_size: 0,
        };
        let mut ret = vec![];
        let mut buff_b = name;
        // Calculate how many lfns we will need.
        const SINGLE_LFN_SIZE: usize = 5 + 6 + 2;
        let required_lfns = name.len().div_ceil(SINGLE_LFN_SIZE) as u8;
        debug!("Required LFNS: {}", required_lfns);
        // Other then for stopping the loop below, it's also useful for the SequenceNumber attribute.

        while (ret.len() + 1) as u8 <= required_lfns {
            let buff = buff_b;
            let (first_set_str, buff) = buff.split_at(min(5, buff.len()));
            let (second_set_str, buff) = buff.split_at(min(6, buff.len()));
            let (third_set_str, buff) = buff.split_at(min(2, buff.len()));
            buff_b = buff;
            info!(
                "LongFileName: full name:'{:?}', first_set: '{:?}' second_set: '{:?}', third_set: '{:?}'",
                name, first_set_str, second_set_str, third_set_str
            );
            let first_set = Self::convert(first_set_str.as_bytes());
            let second_set = Self::convert(second_set_str.as_bytes());
            let third_set = Self::convert(third_set_str.as_bytes());
            info!(
                "final sets: {:?}, {:?}, {:?}",
                first_set, second_set, third_set
            );
            let position = (ret.len() + 1) as u8;
            let mut sequence_number = SequenceNumber::new(position);
            if position == 1 {
                // TODO: I'm not sure it's actually needed...
                //sequence_number.set_first_physical_bit();
            }
            if position == required_lfns {
                info!(
                    "Position == required lfns, setting last bit. Before: {:?}",
                    sequence_number
                );
                sequence_number.set_is_last_bit();
                info!("After: {:?}", sequence_number);
            }
            info!(
                "Sequence Number: {sequence_number:?}, position = {position}, required lfns = {required_lfns}"
            );

            let lfn_entry = LongFileNameEntry {
                sequence_number,
                name_characters: first_set,
                attributes: Attributes(attribute::LFN),
                r#type: 0,
                checksum_dos_filename: checksum,
                second_set_name: second_set,
                _reserved: 0,
                third_set_name: third_set,
            };
            ret.insert(
                0,
                Self::LongFileName(lfn_entry).transmute_into_unknown_dir_entry(),
            );
        }
        ret.push(Self::Regular(regular).transmute_into_unknown_dir_entry());
        Ok(ret)
    }
}

#[cfg(test)]
mod test {
    extern crate std;

    use crate::ClusterId;
    use crate::api::raw_directory_entry::formats::Attributes;
    use crate::api::raw_directory_entry::{
        LongFileNameEntry, RegularDirectoryEntry, VfatDirectoryEntry,
    };

    fn init() {
        unsafe { std::env::set_var("RUST_LOG", "debug") };
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_checksum() {
        assert_eq!(VfatDirectoryEntry::checksum(b"4CS~1   ", b"E  "), 75);
        assert_eq!(VfatDirectoryEntry::checksum(b"8CHARSSI", b"EXT"), 251);
        assert_eq!(VfatDirectoryEntry::checksum(b"8CHARSSI", b"EX "), 199);
        assert_eq!(VfatDirectoryEntry::checksum(b"8CHARSSI", b"E  "), 171);
    }

    #[test]
    fn test_short_filename() {
        init();
        let res = VfatDirectoryEntry::regular_filename_from("4cs....e");
        assert_eq!(res, *b"4CS~1   ");
        assert_eq!(
            VfatDirectoryEntry::get_regular_filename_ext("4cs....e"),
            *b"E  "
        )
    }

    #[test]
    fn test_simple_create_entry() {
        init();
        // Based on the name, it should create different file entry
        // sample -> Regular(RegularDirectoryEntry), padded with 0x20.
        // sample.est -> Regular(RegularDirntr), with est inside file_ext field
        // long-sample-no-ext -> LongFileNameEntry

        let given = VfatDirectoryEntry::new_vfat_entry(
            "4chars.ext",
            ClusterId::new(0),
            Attributes::new_directory(),
            &[],
        )
        .unwrap();
        let expected_regular_name = b"4CHARS~1";
        let expecte_ext = b"EXT";
        assert!(!given.is_empty());

        let lfn: LongFileNameEntry = VfatDirectoryEntry::from(given.get(0).unwrap())
            .into_long_file_name()
            .unwrap();
        let first_set: [u16; 5] = VfatDirectoryEntry::convert(b"4char");
        let second_set: [u16; 6] = VfatDirectoryEntry::convert(b"s.ext");
        let third_set: [u16; 2] = VfatDirectoryEntry::convert(b"");
        assert_eq!({ lfn.name_characters }, first_set);
        assert_eq!({ lfn.second_set_name }, second_set);
        assert_eq!({ lfn.third_set_name }, third_set);

        // last should be regular:
        let get_regular: RegularDirectoryEntry = VfatDirectoryEntry::from(given.get(1).unwrap())
            .into_regular()
            .unwrap();
        assert_eq!(&get_regular.file_name, expected_regular_name);
        assert_eq!(&get_regular.file_ext, expecte_ext);
    }

    #[test]
    fn test_long_entry() {
        init();
        let name = "a-super-very-long-file-name-entry.txt";
        println!("Name: {}", name);
        let mut given = VfatDirectoryEntry::new_vfat_entry(
            name,
            ClusterId::new(0),
            Attributes::new_directory(),
            &[],
        )
        .unwrap();
        given
            .clone()
            .into_iter()
            .map(VfatDirectoryEntry::from)
            .for_each(|entry| println!("Entry: {:?}", entry));
        assert_eq!(given.len(), 4);
        // -----

        let lfn = VfatDirectoryEntry::from(given.remove(0))
            .into_long_file_name()
            .unwrap();
        assert_eq!(VfatDirectoryEntry::convert(b"e-ent-"), {
            lfn.name_characters
        });
        assert_eq!(VfatDirectoryEntry::convert(b"ry.txt"), {
            lfn.second_set_name
        });
        assert_eq!(VfatDirectoryEntry::convert(b""), { lfn.third_set_name });
        // ---

        let lfn = VfatDirectoryEntry::from(given.remove(0))
            .into_long_file_name()
            .unwrap();
        assert_eq!(VfatDirectoryEntry::convert(b"long-"), {
            lfn.name_characters
        });
        assert_eq!(VfatDirectoryEntry::convert(b"file-n"), {
            lfn.second_set_name
        });
        assert_eq!(VfatDirectoryEntry::convert(b"am"), { lfn.third_set_name });

        let lfn = VfatDirectoryEntry::from(given.remove(0))
            .into_long_file_name()
            .unwrap();
        assert_eq!(VfatDirectoryEntry::convert(b"a-sup"), {
            lfn.name_characters
        });
        assert_eq!(VfatDirectoryEntry::convert(b"er-ver"), {
            lfn.second_set_name
        });
        assert_eq!(VfatDirectoryEntry::convert(b"y-"), { lfn.third_set_name });

        VfatDirectoryEntry::from(given.remove(0))
            .into_regular()
            .unwrap();
    }

    #[test]
    fn test_short_filename_with_tail() {
        // ~1: prefix_len=6, base "4cs" (3 chars) → "4CS~1   "
        assert_eq!(
            VfatDirectoryEntry::regular_filename_from_with_tail("4cs....e", 1),
            *b"4CS~1   "
        );
        // ~2: prefix_len=6, base "4cs" (3 chars) → "4CS~2   "
        assert_eq!(
            VfatDirectoryEntry::regular_filename_from_with_tail("4cs....e", 2),
            *b"4CS~2   "
        );
        // ~10: prefix_len=5, base "4cs" (3 chars) → "4CS~10  "
        assert_eq!(
            VfatDirectoryEntry::regular_filename_from_with_tail("4cs....e", 10),
            *b"4CS~10  "
        );
        // ~100: prefix_len=4, base "longname" (8 chars, take 4) → "LONG~100"
        assert_eq!(
            VfatDirectoryEntry::regular_filename_from_with_tail("longname.txt", 100),
            *b"LONG~100"
        );
    }

    #[test]
    fn test_new_vfat_entry_avoids_collision() {
        init();
        // "4chars.ext" base is "4chars" → "4CHARS~1". Simulate that as existing.
        let existing = vec![*b"4CHARS~1"];
        let entries = VfatDirectoryEntry::new_vfat_entry(
            "4chars.ext",
            ClusterId::new(0),
            Attributes::new_directory(),
            &existing,
        )
        .unwrap();
        // The regular entry should use ~2 instead of ~1
        let regular: RegularDirectoryEntry = VfatDirectoryEntry::from(entries.last().unwrap())
            .into_regular()
            .unwrap();
        assert_eq!(&regular.file_name, b"4CHARS~2");
    }

    #[test]
    fn test_new_vfat_entry_no_collision() {
        init();
        // No existing entries — should use ~1
        let entries = VfatDirectoryEntry::new_vfat_entry(
            "4chars.ext",
            ClusterId::new(0),
            Attributes::new_directory(),
            &[],
        )
        .unwrap();
        let regular: RegularDirectoryEntry = VfatDirectoryEntry::from(entries.last().unwrap())
            .into_regular()
            .unwrap();
        assert_eq!(&regular.file_name, b"4CHARS~1");
    }

    #[test]
    fn test_name_too_long_returns_error() {
        let long_name = "a".repeat(256);
        let result = VfatDirectoryEntry::new_vfat_entry(
            &long_name,
            ClusterId::new(0),
            Attributes::new_directory(),
            &[],
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(
                err,
                crate::error::VfatRsError::NameTooLong { length: 256, .. }
            ),
            "Expected NameTooLong, got: {:?}",
            err
        );
    }

    #[test]
    fn test_max_length_name_succeeds() {
        let name = "a".repeat(255);
        let result = VfatDirectoryEntry::new_vfat_entry(
            &name,
            ClusterId::new(0),
            Attributes::new_directory(),
            &[],
        );
        assert!(result.is_ok());
    }
}
