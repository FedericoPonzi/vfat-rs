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
    /// Set the year field (clamped to 1980 minimum per VFAT spec).
    pub fn set_year(&mut self, year: u32) -> &mut Self {
        // 1980 is the min in vfat timestamps.
        self.set_value(max(year, 1980) % 1980, VfatTimestamp::YEAR)
    }
    /// Set the seconds field (VFAT has 2-second resolution).
    pub fn set_seconds(&mut self, seconds: u32) -> &mut Self {
        // VFAT has a 2-second resolution
        self.set_value(seconds / 2, VfatTimestamp::SECONDS)
    }
    /// Returns the year (1980–2107).
    pub fn year(&self) -> u32 {
        self.get_value(Self::YEAR) + 1980_u32
    }
    /// Returns the month (1–12).
    pub fn month(&self) -> u32 {
        self.get_value(Self::MONTH)
    }
    /// Returns the day of the month (1–31).
    pub fn day(&self) -> u32 {
        self.get_value(Self::DAY)
    }
    /// Returns the hour (0–23).
    pub fn hour(&self) -> u32 {
        self.get_value(Self::HOURS)
    }
    /// Returns the minute (0–59).
    pub fn minute(&self) -> u32 {
        self.get_value(Self::MINUTES)
    }
    /// Seconds are stored as number of 2-second intervals.
    /// Range: 0..29 29 represents 58 seconds
    pub fn second(&self) -> u32 {
        self.get_value(Self::SECONDS) * 2
    }
}

type UnixTimestamp = u64;

impl From<UnixTimestamp> for VfatTimestamp {
    fn from(value: u64) -> Self {
        let is_leap_year = |year| -> bool { (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 };
        const SECONDS_IN_MINUTE: u32 = 60;
        const SECONDS_IN_HOUR: u32 = 60 * SECONDS_IN_MINUTE;
        const SECONDS_IN_DAY: u32 = 24 * SECONDS_IN_HOUR;

        let mut remaining_seconds = value as u32;

        let mut days_since_1970 = remaining_seconds / SECONDS_IN_DAY;
        remaining_seconds %= SECONDS_IN_DAY;

        let mut year = 1970u32;
        let mut day_count;

        loop {
            day_count = if is_leap_year(year) { 366 } else { 365 };
            if days_since_1970 >= day_count {
                days_since_1970 -= day_count;
                year += 1;
            } else {
                break;
            }
        }

        let mut month = 1u32;
        let days_in_month = [
            31,
            28 + (is_leap_year(year) as u32),
            31,
            30,
            31,
            30,
            31,
            31,
            30,
            31,
            30,
            31,
        ];

        while days_since_1970 >= days_in_month[(month - 1) as usize] {
            days_since_1970 -= days_in_month[(month - 1) as usize];
            month += 1;
        }

        let day = days_since_1970 + 1;
        let hour = remaining_seconds / SECONDS_IN_HOUR;
        remaining_seconds %= SECONDS_IN_HOUR;
        let minute = remaining_seconds / SECONDS_IN_MINUTE;
        let second = remaining_seconds % SECONDS_IN_MINUTE;

        let mut timestamp = VfatTimestamp::new(0);

        timestamp
            // 1980 is the min in vfat timestamps.
            .set_year(year)
            .set_value(month, VfatTimestamp::MONTH)
            .set_value(day, VfatTimestamp::DAY)
            .set_value(hour, VfatTimestamp::HOURS)
            .set_value(minute, VfatTimestamp::MINUTES)
            .set_seconds(second); // VFAT has a 2-second resolution

        timestamp
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

    #[test]
    fn test_vfattimestamp_from_unixtimestamp() {
        // TODO
    }
}
