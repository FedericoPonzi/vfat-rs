use alloc::string::String;
use core::iter;

/// A simple implementation of PathBuf. Vfat uses utf8/utf16 for encoding: https://wiki.gentoo.org/wiki/FAT/en#UTF-8.2FUTF-16_character_hardware_bugs
/// therefore it's ok to use a String as a baking data structure.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct PathBuf(pub String);

impl PathBuf {
    /// Create a new `PathBuf` from a string-like value.
    pub fn new<S: AsRef<str>>(path: S) -> Self {
        Self(String::from(path.as_ref()))
    }
    /// Returns an iterator over the path components.
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        iter::once("/").chain(self.0[1..].split_terminator('/'))
    }
    /// Returns the path as a string slice.
    pub fn to_str(&self) -> &str {
        self.0.as_str()
    }
    /// Returns a displayable string slice.
    pub fn display(&self) -> &str {
        self.to_str()
    }
    /// Returns `true` if the path starts with `/`.
    pub fn is_absolute(&self) -> bool {
        self.0.starts_with('/')
    }
    /// Returns the parent directory path, or None if this is the root.
    pub fn parent(&self) -> Option<PathBuf> {
        if self.0 == "/" {
            return None;
        }
        let trimmed = self.0.trim_end_matches('/');
        match trimmed.rfind('/') {
            Some(0) => Some(PathBuf::from("/")),
            Some(pos) => Some(PathBuf::from(&trimmed[..pos])),
            None => None,
        }
    }
    /// Returns the final component of the path.
    pub fn file_name(&self) -> Option<&str> {
        let trimmed = self.0.trim_end_matches('/');
        if trimmed.is_empty() {
            return None;
        }
        match trimmed.rfind('/') {
            Some(pos) => {
                let name = &trimmed[pos + 1..];
                if name.is_empty() {
                    None
                } else {
                    Some(name)
                }
            }
            None => Some(trimmed),
        }
    }
    /// Returns true if `self` starts with the given base path.
    pub fn starts_with(&self, base: &PathBuf) -> bool {
        let self_trimmed = self.0.trim_end_matches('/');
        let base_trimmed = base.0.trim_end_matches('/');
        if base_trimmed == "/" {
            return self_trimmed.starts_with('/');
        }
        self_trimmed == base_trimmed
            || self_trimmed.starts_with(&alloc::format!("{}/", base_trimmed))
    }
}
impl core::fmt::Display for PathBuf {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl PartialEq<String> for &PathBuf {
    fn eq(&self, other: &String) -> bool {
        other.as_str() == self.0.as_str()
    }
}
impl PartialEq<&str> for &PathBuf {
    fn eq(&self, other: &&str) -> bool {
        *other == self.0.as_str()
    }
}

impl From<&str> for PathBuf {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}
impl From<String> for PathBuf {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}
