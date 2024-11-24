use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{self, Display};
use std::ops::{Add, Sub};
use time::macros::format_description;
use time::{format_description::well_known::Rfc3339, Month, OffsetDateTime};

/// Date is datetime in UTC.
/// All timestamps in Devy are in UTC.
/// Any representation of time for the user is determined by using the user's current time.
#[derive(Hash, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Date {
    pub year: i32,
    pub month: Month,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl Date {
    /// Returns a new Date instance representing the given time in UTC.
    pub fn new(year: i32, month: Month, day: u8, hour: u8, minute: u8, second: u8) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        }
    }

    /// Returns a new Date instance representing the current time in UTC.
    pub fn now() -> Self {
        let offset_datetime = OffsetDateTime::now_utc();
        Self {
            year: offset_datetime.year(),
            month: offset_datetime.month(),
            day: offset_datetime.day(),
            hour: offset_datetime.hour(),
            minute: offset_datetime.minute(),
            second: offset_datetime.second(),
        }
    }

    /// Returns if the date is within the given seconds of another date.
    pub fn is_around(&self, other: &Self, seconds: u32) -> bool {
        let self_seconds = self.hour as u32 * 3600 + self.minute as u32 * 60 + self.second as u32;
        let other_seconds =
            other.hour as u32 * 3600 + other.minute as u32 * 60 + other.second as u32;
        (self.year == other.year && self.month == other.month && self.day == other.day)
            && (self_seconds as i32 - other_seconds as i32).abs() <= seconds as i32
    }

    fn to_rfc3339(&self) -> String {
        let offset_datetime = OffsetDateTime::from_unix_timestamp(0)
            .unwrap()
            .replace_date(time::Date::from_calendar_date(self.year, self.month, self.day).unwrap())
            .replace_time(time::Time::from_hms(self.hour, self.minute, self.second).unwrap());
        offset_datetime.format(&Rfc3339).unwrap()
    }

    fn from_rfc3339(s: &str) -> Result<Self, time::error::Parse> {
        let format = format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]Z");
        let offset_datetime = OffsetDateTime::parse(s, &format)?;
        Ok(Self {
            year: offset_datetime.year(),
            month: offset_datetime.month(),
            day: offset_datetime.day(),
            hour: offset_datetime.hour(),
            minute: offset_datetime.minute(),
            second: offset_datetime.second(),
        })
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.to_rfc3339().cmp(&other.to_rfc3339()))
    }
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = self.to_rfc3339();
        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DateVisitor;

        impl<'de> Visitor<'de> for DateVisitor {
            type Value = Date;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string in the format YYYY-MM-DDThh:mm:ss.ssZ")
            }

            fn visit_str<E>(self, value: &str) -> Result<Date, E>
            where
                E: de::Error,
            {
                Date::from_rfc3339(value).map_err(de::Error::custom)
            }
        }

        deserializer.deserialize_str(DateVisitor)
    }
}

impl Add<i64> for Date {
    type Output = Self;

    fn add(self, seconds: i64) -> Self {
        let offset_datetime = OffsetDateTime::from_unix_timestamp(0)
            .unwrap()
            .replace_date(time::Date::from_calendar_date(self.year, self.month, self.day).unwrap())
            .replace_time(time::Time::from_hms(self.hour, self.minute, self.second).unwrap())
            + time::Duration::seconds(seconds);
        Self {
            year: offset_datetime.year(),
            month: offset_datetime.month(),
            day: offset_datetime.day(),
            hour: offset_datetime.hour(),
            minute: offset_datetime.minute(),
            second: offset_datetime.second(),
        }
    }
}

impl Sub<i64> for Date {
    type Output = Self;

    fn sub(self, seconds: i64) -> Self {
        self.add(-1 * seconds)
    }
}

impl From<sqlx::types::time::OffsetDateTime> for Date {
    fn from(date: sqlx::types::time::OffsetDateTime) -> Self {
        Date {
            year: date.year(),
            month: date.month(),
            day: date.day(),
            hour: date.hour(),
            minute: date.minute(),
            second: date.second(),
        }
    }
}

impl From<Date> for sqlx::types::time::OffsetDateTime {
    fn from(date: Date) -> Self {
        sqlx::types::time::OffsetDateTime::from_unix_timestamp(0)
            .unwrap()
            .replace_date(time::Date::from_calendar_date(date.year, date.month, date.day).unwrap())
            .replace_time(time::Time::from_hms(date.hour, date.minute, date.second).unwrap())
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
            self.year, self.month as u8, self.day, self.hour, self.minute, self.second
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_date_math() {
        let date = Date::new(1994, Month::February, 25, 10, 0, 0);
        let past_date = Date::new(1994, Month::February, 25, 9, 59, 0);
        let future_date = Date::new(1994, Month::February, 25, 10, 1, 0);

        // Test addition
        assert_eq!(future_date, date + 60);

        // Test subtraction
        assert_eq!(past_date, date - 60);
    }
}
