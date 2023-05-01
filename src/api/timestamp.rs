//! * The years are represented as an offset from 1980, using 7 bits, which allows representation of years from 1980 to 2107./
//! * Months are represented using 4 bits, which allows representation of 12 months (1 to 12).
//! * Days are represented using 5 bits, which allows representation of 31 days (1 to 31).
//! * Hours are represented using 5 bits, which allows representation of 24 hours (0 to 23).
//! * Minutes are represented using 6 bits, which allows representation of 60 minutes (0 to 59).
//! * Seconds are represented using 5 bits, which allows representation of 30 intervals (0 to 29) because the resolution is 2 seconds.

use crate::defbit;
use core::cmp::max;
use core::fmt::Display;

/// Tenths of a second. Range 0-199 inclusive,
/// as represented in FAT32 on-disk structures.
pub type Milliseconds = u8;

defbit!(
    VfatTimestamp,
    u32,
    [
        YEAR[31 - 25],
        MONTH[24 - 21],
        DAY[20 - 16],
        HOURS[15 - 11],
        MINUTES[10 - 5],
        SECONDS[4 - 0],
    ]
);

///15-11 Hours (0-23)
// 10-5 Minutes (0-59)
// 4-0 Seconds/2 (0-29)
impl VfatTimestamp {
    // year is special as it has a min of 1980. Encapsulate logic for setting the new value.
    pub fn set_year(&mut self, year: u32) -> &mut Self {
        // 1980 is the min in vfat timestamps.
        self.set_value(max(year, 1980) % 1980, VfatTimestamp::YEAR)
    }
    pub fn set_seconds(&mut self, seconds: u32) -> &mut Self {
        // VFAT has a 2-second resolution
        self.set_value(seconds / 2, VfatTimestamp::SECONDS)
    }
    pub fn year(&self) -> u32 {
        self.get_value(Self::YEAR) + 1980_u32
    }
    pub fn month(&self) -> u32 {
        self.get_value(Self::MONTH)
    }
    pub fn day(&self) -> u32 {
        self.get_value(Self::DAY)
    }
    pub fn hour(&self) -> u32 {
        self.get_value(Self::HOURS)
    }
    pub fn minute(&self) -> u32 {
        self.get_value(Self::MINUTES)
    }
    /// Seconds are stored as number of 2-second intervals.
    /// Range: 0..29 29 represents 58 seconds
    pub fn second(&self) -> u32 {
        self.get_value(Self::SECONDS) * 2
    }
}
impl Display for VfatTimestamp {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.year(),
            self.month(),
            self.day(),
            self.hour(),
            self.minute(),
            self.second()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vfat_timestamp() {
        let timestamp = VfatTimestamp::new(0);
        assert_eq!(timestamp.year(), 1980);
        assert_eq!(timestamp.month(), 0);
        assert_eq!(timestamp.day(), 0);
        assert_eq!(timestamp.hour(), 0);
        assert_eq!(timestamp.minute(), 0);
        assert_eq!(timestamp.second(), 0);

        let mut timestamp = VfatTimestamp::new(0);
        timestamp
            .set_value(0u32, VfatTimestamp::YEAR)
            .set_value(0u32, VfatTimestamp::MONTH)
            .set_value(0u32, VfatTimestamp::DAY)
            .set_value(0u32, VfatTimestamp::HOURS)
            .set_value(0u32, VfatTimestamp::MINUTES)
            .set_value(0u32, VfatTimestamp::SECONDS);
        assert_eq!(timestamp, VfatTimestamp::new(0));

        let mut timestamp = VfatTimestamp::new(0);
        timestamp
            .set_value(42u32, VfatTimestamp::YEAR)
            .set_value(6u32, VfatTimestamp::MONTH)
            .set_value(7u32, VfatTimestamp::DAY)
            .set_value(5u32, VfatTimestamp::HOURS)
            .set_value(6u32, VfatTimestamp::MINUTES)
            .set_value(8u32, VfatTimestamp::SECONDS);

        assert_eq!(timestamp.year(), 2022);
        assert_eq!(timestamp.month(), 6);
        assert_eq!(timestamp.day(), 7);
        assert_eq!(timestamp.hour(), 5);
        assert_eq!(timestamp.minute(), 6);
        assert_eq!(timestamp.second(), 16);
    }
}
