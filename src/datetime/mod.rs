//!
//! Provides 14 functions to return mock date and time data.
//!
//! # Examples
//!
//! ```rust
//! use mockd::datetime;
//!
//!     let data = datetime::month(); // month: 10
//!     let data = datetime::day(); // day: 10
//!     let data = datetime::week_day(); // week_day: 6
//!     let data = datetime::year(); // year: 1986
//!     let data = datetime::hour(); // hour: 10
//!     let data = datetime::minute(); // minute: 10
//!     let data = datetime::second(); // second: 10
//!     let data = datetime::nanosecond(); // nanosecond: 959678991
//!     let data = datetime::timezone(); // timezone: SA Pacific Standard Time
//!     let data = datetime::timezone_full(); // timezone_full: (UTC-04:00) Atlantic Time    (Canada)
//!     let data = datetime::timezone_abv(); // timezone_abv: BST
//!     let data = datetime::timezone_offset(); // timezone_offset: 13
//!     let data = datetime::date_range(
//!                 "2005-04-23T19:30:12Z".to_string(),
//!                 "2019-10-02T19:30:12Z".to_string()); // date_range: 1979-01-06 23:03:10.918301212 UTC
//!     let data = datetime::date(); // date: 1979-01-06 23:03:10.918301212 UTC
//! ```
//!
//! # Feature
//!
//! Requires the "datetime" feature.
//!

use crate::misc;
use chrono::{DateTime, Datelike, NaiveDateTime, Utc};

pub(crate) mod data;

/// Generate a random month.
///
/// Returns a month by number as a string.
/// Values range from 1-12
///
/// # Example
///
/// ```rust
/// let month = mockd::datetime::month();
///
/// println!("Month: {}", month);
/// ```
///
/// # Feature
///
/// Requires the "datetime" feature.
///
pub fn month() -> String {
    misc::random::<i8>(1, 12).to_string()
}

/// Generate a random day.
///
/// A returns a valid number for a day (in any month) as a string.
/// Values range from 1-28.
///
/// # Example
///
/// ```rust
/// let day = mockd::datetime::day();
///
/// println!("Day: {}", day);
/// ```
///
/// # Feature
///
/// Requires the "datetime" feature.
///
pub fn day() -> String {
    misc::random::<i8>(1, 28).to_string()
}

/// Generate a random week day.
///
/// A returns a valid number for a week day as a string.
/// Values range from 0-6.
///
/// # Example
///
/// ```rust
/// let day = mockd::datetime::day();
///
/// println!("Day: {}", day);
/// ```
///
/// # Feature
///
/// Requires the "datetime" feature.
///
pub fn week_day() -> String {
    misc::random::<i8>(0, 6).to_string()
}

/// Generate a random year.
///
/// A returns a valid number for a year as a string.
/// Values range from 1980 to the current year..
///
/// # Example
///
/// ```rust
/// let year = mockd::datetime::year();
///
/// println!("Year: {}", year);
/// ```
///
/// # Feature
///
/// Requires the "datetime" feature.
///
pub fn year() -> String {
    misc::random::<i32>(1980, Utc::now().year()).to_string()
}

/// Generate a random hour.
///
/// A returns a valid number for an hour as a string.
/// Values range from 0-23.
///
/// # Example
///
/// ```rust
/// let hour = mockd::datetime::hour();
///
/// println!("Hour: {}", hour);
/// ```
///
/// # Feature
///
/// Requires the "datetime" feature.
///
pub fn hour() -> String {
    misc::random::<i8>(0, 23).to_string()
}

/// Generate a random minute.
///
/// A returns a valid number for a minute as a string.
/// Values range from 0-59.
///
/// # Example
///
/// ```rust
/// let minute = mockd::datetime::minute();
///
/// println!("Minute: {}", minute);
/// ```
///
/// # Feature
///
/// Requires the "datetime" feature.
///
pub fn minute() -> String {
    misc::random::<i8>(0, 59).to_string()
}

/// Generate a random second.
///
/// A returns a valid number for a second as a string.
/// Values range from 0-59.
///
/// # Example
///
/// ```rust
/// let second = mockd::datetime::second();
///
/// println!("Second: {}", second);
/// ```
///
/// # Feature
///
/// Requires the "datetime" feature.
///
pub fn second() -> String {
    misc::random::<i8>(0, 59).to_string()
}

/// Generate a random nanosecond.
///
/// A returns a valid number for a nanosecond as a string.
/// Values range from 0-999999999.
///
/// # Example
///
/// ```rust
/// let nanosecond = mockd::datetime::nanosecond();
///
/// println!("Nanosecond: {}", nanosecond);
/// ```
///
/// # Feature
///
/// Requires the "datetime" feature.
///
pub fn nanosecond() -> String {
    misc::random::<i64>(0, 999_999_999).to_string()
}

/// Generate a random timezone name.
///
/// A returns a timezone name.
/// Sample values:
/// * Mountain Standard Time
/// * Central Standard Time (Mexico)
///
/// # Example
///
/// ```rust
/// let timezone = mockd::datetime::timezone();
///
/// println!("Timezone: {}", timezone);
/// ```
///
/// # Feature
///
/// Requires the "datetime" feature.
///
pub fn timezone() -> String {
    misc::random_data(data::TEXT).to_string()
}

/// Generate a random full timezone description.
///
/// A returns a full timezone description.
/// Sample values:
/// * (UTC-07:00) Mountain Time (US & Canada)
/// * (UTC-06:00) Guadalajara, Mexico City, Monterrey
/// * (UTC-04:00) Georgetown, La Paz, Manaus, San Juan
///
/// # Example
///
/// ```rust
/// let timezone_full = mockd::datetime::timezone_full();
///
/// println!("Full timezone description: {}", timezone_full);
/// ```
///
/// # Feature
///
/// Requires the "datetime" feature.
///
pub fn timezone_full() -> String {
    misc::random_data(data::FULL).to_string()
}

/// Generate a random timezone abbreviation.
///
/// A returns a timezone abbreviation.
/// Sample values:
/// * MDT
/// * CCST
/// * GDT
///
/// # Example
///
/// ```rust
/// let timezone_abr = mockd::datetime::timezone_abv();
///
/// println!("Timezone abbreviation: {}", timezone_abr);
/// ```
///
/// # Feature
///
/// Requires the "datetime" feature.
///
pub fn timezone_abv() -> String {
    misc::random_data(data::ABR).to_string()
}

/// Generate a random timezone offset.
///
/// A returns a timezone offset as a string.
/// Sample values:
/// * -8
/// * -2.5
/// * 3
/// * 9.5
///
/// # Example
///
/// ```rust
/// let timezone_offset = mockd::datetime::timezone_offset();
///
/// println!("Timezone offset: {}", timezone_offset);
/// ```
///
/// # Feature
///
/// Requires the "datetime" feature.
///
pub fn timezone_offset() -> String {
    misc::random_data(data::OFFSET).to_string()
}

/// Generate a random date from within a range.
///
/// # Example
///
/// ```rust
/// let random_date = mockd::datetime::date_range(
///                     "2005-04-23T19:30:12Z".to_string(),
///                     "2019-10-12T19:30:12Z".to_string());
///
/// println!("Random date from range: {}",random_date);
/// ```
///
/// # Feature
///
/// Requires the "datetime" feature.
///
pub fn date_range(min: String, max: String) -> DateTime<Utc> {
    // RFC3339
    let min_nano = DateTime::parse_from_rfc3339(&min)
        .unwrap()
        .timestamp_nanos();
    let max_nano = DateTime::parse_from_rfc3339(&max)
        .unwrap()
        .timestamp_nanos();
    let ns = misc::random(min_nano, max_nano - 10_000_000_000);
    let secs = (ns / 1_000_000_000) as i64;
    let mut nsecs = (ns - (secs * 1_000_000_000)) as u32;

    // This case will cause the `NaiveDateTime::from_timestamp` function to panic.
    // So we just roll it back to the maximum allowed value.
    if nsecs >= 2_000_000_000 {
        nsecs = 1_999_999_999;
    }

    DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp_opt(secs, nsecs as u32).unwrap(),
        Utc,
    )
}

/// Generate a random date.
///
/// # Example
///
/// ```rust
/// let random_date = mockd::datetime::date();
///
/// println!("Random date from range: {}",random_date);
/// ```
///
/// # Feature
///
/// Requires the "datetime" feature.
///
pub fn date() -> DateTime<Utc> {
    date_range(
        "1970-01-01T16:39:57-08:00".to_string(),
        Utc::now().to_rfc3339(),
    )
}

#[cfg(test)]
mod tests {
    use crate::datetime;
    use crate::testify::exec_mes;

    #[test]
    fn timezone() {
        exec_mes("datetime::timezone", datetime::timezone);
    }

    #[test]
    fn timezone_full() {
        exec_mes("datetime::timezone_full", datetime::timezone_full);
    }

    #[test]
    fn timezone_abv() {
        exec_mes("datetime::timezone_abv", datetime::timezone_abv);
    }

    #[test]
    fn timezone_offset() {
        exec_mes("datetime::timezone_offset", datetime::timezone_offset);
    }

    #[test]
    fn date() {
        let data1 = datetime::date();
        let data2 = datetime::date();
        assert_ne!(data1, data2);
    }
}
