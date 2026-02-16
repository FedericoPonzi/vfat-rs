use core::fmt;
use core::ops::Add;

/// The sector's index on the block device.
/// TODO: this is fine for now, but the wrapped type should change to usize
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd)]
pub struct SectorId(pub u32);

impl From<u32> for SectorId {
    fn from(v: u32) -> Self {
        SectorId(v)
    }
}

impl fmt::Display for SectorId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Add for SectorId {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        SectorId(self.0 + rhs.0)
    }
}

impl Add<u32> for SectorId {
    type Output = SectorId;

    fn add(self, other: u32) -> Self::Output {
        SectorId(self.0 + other)
    }
}

#[cfg(test)]
mod test {
    use crate::SectorId;

    #[test]
    fn test_sector_sum() {
        assert_eq!(SectorId(1) + SectorId(1), SectorId(2));
        assert_eq!(SectorId(1) + 2, SectorId(3));
    }
}
