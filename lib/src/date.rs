use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use time::macros::format_description;
use time::{format_description::well_known::Rfc3339, Month, OffsetDateTime};

/// Date is datetime in UTC.
#[derive(Debug, Clone, PartialEq)]
pub struct Date {
    pub year: i32,
    pub month: Month,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl Date {
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
