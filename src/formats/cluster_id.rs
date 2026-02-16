use core::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) struct ClusterId(u32);
impl From<ClusterId> for u32 {
    fn from(cid: ClusterId) -> Self {
        cid.0
    }
}

impl ClusterId {
    pub fn new(id: u32) -> Self {
        ClusterId(id)
    }

    // Returns the high and the low part of this cluster id
    pub fn into_high_low(self) -> (u16, u16) {
        ((self.0 >> 16) as u16, (self.0 & 0xFFFF) as u16)
    }

    // Builds back the clusterid from high and low parts.
    pub fn from_high_low(high: u16, low: u16) -> Self {
        let raw_bytes: [u8; 4] = [
            (high >> 8) as u8,
            (high & 0xFF) as u8,
            (low >> 8) as u8,
            (low & 0xFF) as u8,
        ];
        ClusterId::new(u32::from_be_bytes(raw_bytes))
    }
}

impl fmt::Display for ClusterId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use crate::ClusterId;

    #[test]
    fn test_high_low() {
        let n = 0b1000_1000_0001_0001_1000_1000_0001_0001;
        let high = 0b1000_1000_0001_0001;
        let low = high;
        assert_eq!((high, low), ClusterId::new(n).into_high_low(), "i: {}", n);
        assert_eq!(ClusterId::from_high_low(high, low).0, n);
    }
}
