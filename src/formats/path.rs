use alloc::string::String;
use core::iter;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct PathBuf(pub String);

impl PathBuf {
    pub fn new<S: AsRef<str>>(path: S) -> Self {
        Self(String::from(path.as_ref()))
    }
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        iter::once("/").chain(self.0[1..].split_terminator('/'))
    }
    pub fn to_str(&self) -> &str {
        self.0.as_str()
    }
    pub fn display(&self) -> &str {
        self.to_str()
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
