//! Provides 16 functions to return mock address data.
//!
//!
//! # Examples
//!
//! ```rust
//! use mockd::address;
//!
//!     let data = address::info(); // address::Info struct
//!     let data = address::street(); // street: 1128 South North Dakota borough
//!     let data = address::street_number(); // street_number: 3155
//!     let data = address::street_prefix(); // street_prefix: Port
//!     let data = address::street_name(); // street_name: Kansas
//!     let data = address::street_suffix(); // street_suffix: mouth
//!     let data = address::city(); // city: Schmelerburgh
//!     let data = address::state(); // state: Kentucky
//!     let data = address::state_abr(); // state_abr: WA
//!     let data = address::zip(); // zip: 75221
//!     let data = address::country(); // country: Romania
//!     let data = address::country_abr(); // country_abr: BI
//!     let data = address::latitude(); // latitude: -69.14192
//!     let data = address::latitude_in_range(-30.0 as f32, 30.0 as f32); // latitude_in_range: -18.35571
//!     let data = address::longitude(); // longitude: 113.12952
//!     let data = address::longitude_in_range(-30.0 as f32, 30.0 as f32); // longitude_in_range: -16.484156
//! ```
//! # Feature
//!
//! Requires the "address" feature.
//!

use crate::misc;
use crate::name;

pub(crate) mod data;

/// Information that may be required for testing about an address.
///
/// # Example
///
/// ```rust
/// use mockd::address::Info;
///
/// let mock_address = mockd::address::info();
///
/// println!("Address Info: {:#?}", mock_address);
/// ```
///
/// # Feature
///
/// Requires the "address" feature.
///
#[derive(Debug)]
pub struct Info {
    /// Single string for address constructed from the street, city, state and zip address elements.
    address: String,
    street: String,
    city: String,
    state: String,
    zip: String,
    country: String,
    latitude: f32,
    longitude: f32,
}

/// Generate and return the Info structure containing address data.
///
/// # Example
///
/// ```rust
/// use mockd::address::Info;
///
/// let mock_address = mockd::address::info();
///
/// println!("Address Info: {:#?}", mock_address);
/// ```
///
/// # Feature
///
/// Requires the "address" feature.
///
pub fn info() -> Info {
    Info {
        address: format!("{}, {}, {} {}", street(), city(), state(), zip()),
        street: street(),
        city: city(),
        state: state(),
        zip: zip(),
        country: country(),
        latitude: latitude(),
        longitude: longitude(),
    }
}

/// Generate a random street name.
///
/// The street name consists of up to four components:
/// * street_number
/// * street_prefix
/// * street_name
/// * street_suffix
///
/// The function randomly returns one of two formats:
/// * <street_number> <street_prefix> <street_name> <street_suffix>
/// * <street_number> <street_name> <street_suffix>
///
/// # Example
///
/// ```rust
/// let street = mockd::address::street();
///
/// println!("Street: {}", street);
/// ```
///
/// # Feature
///
/// Requires the "address" feature.
///
pub fn street() -> String {
    match misc::random::<i64>(1, 2) {
        1 => {
            format!(
                "{} {} {} {}",
                street_number(),
                street_prefix(),
                street_name(),
                street_suffix()
            )
        }
        2 => format!("{} {} {}", street_number(), street_name(), street_suffix()),
        _ => "impossible".to_string(),
    }
}

/// Generate a random street number.
///
/// # Example
///
/// ```rust
/// let street_number = mockd::address::street_number();
///
/// println!("Street number: {}", street_number);
/// ```
///
/// # Feature
///
/// Requires the "address" feature.
///
pub fn street_number() -> String {
    misc::replace_with_numbers(misc::random_data(data::NUMBER).to_string())
}

/// Generate a random street prefix.
///
/// The street prefix is a word that may form the first part of a street name.
/// E.g. "North" or "Lake"
///
/// # Example
///
/// ```rust
/// use mockd::address::Info;
///
/// let street_prefix = mockd::address::street_prefix();
///
/// println!("Street prefix: {}", street_prefix);
/// ```
///
/// # Feature
///
/// Requires the "address" feature.
///
pub fn street_prefix() -> String {
    misc::random_data(data::STREET_PREFIX).to_string()
}

/// Generate a random street name.
///
/// ```rust
/// let street_name = mockd::address::street_name();
///
/// println!("Street name: {}", street_name);
/// ```
///
/// # Feature
///
/// Requires the "address" feature.
///
pub fn street_name() -> String {
    misc::random_data(data::STREET_NAME).to_string()
}

/// Generate a random street suffix.
///
/// The street suffix is a word that may form the last part of street name.
/// E.g. "town" or "view"
///
/// # Example
///
/// ```rust
/// let street_suffix = mockd::address::street_suffix();
///
/// println!("Street suffix: {}", street_suffix);
/// ```
///
/// # Feature
///
/// Requires the "address" feature.
///
pub fn street_suffix() -> String {
    misc::random_data(data::STREET_SUFFIX).to_string()
}

/// Generate a random city.
///
/// # Example
///
/// ```rust
/// let city = mockd::address::city();
///
/// println!("City: {}", city);
/// ```
///
/// # Feature
///
/// Requires the "address" feature.
///
pub fn city() -> String {
    match misc::random::<i64>(1, 3) {
        1 => format!("{}{}", name::first(), street_suffix()),
        2 => format!("{}{}", name::last(), street_suffix()),
        3 => format!("{} {}", street_prefix(), name::last()),
        _ => "impossible".to_string(),
    }
}

/// Generate a US state name.
///
/// # Example
///
/// ```rust
/// let state = mockd::address::state();
///
/// println!("State: {}", state);
/// ```
///
/// # Feature
///
/// Requires the "address" feature.
///
pub fn state() -> String {
    misc::random_data(data::STATE).to_string()
}

/// Generate a US state abbreviation.
///
/// # Example
///
/// ```rust
/// let state_abr = mockd::address::state_abr();
///
/// println!("State (abbreviation): {}", state_abr);
/// ```
///
/// # Feature
///
/// Requires the "address" feature.
///
pub fn state_abr() -> String {
    misc::random_data(data::STATE_ABR).to_string()
}

/// Generate a random zip code.
///
/// # Example
///
/// ```rust
/// let zip = mockd::address::zip();
///
/// println!("Zip: {}", zip);
/// ```
///
/// # Feature
///
/// Requires the "address" feature.
///
pub fn zip() -> String {
    misc::replace_with_numbers(misc::random_data(data::ZIP).to_string())
}

/// Generate a random country.
///
/// # Example
///
/// ```rust
/// let country = mockd::address::country();
///
/// println!("Country: {}", country);
/// ```
///
/// # Feature
///
/// Requires the "address" feature.
///
pub fn country() -> String {
    misc::random_data(data::COUNTRY).to_string()
}

/// Generate a country abbreviation
///
/// # Example
///
/// ```rust
/// let country_abr = mockd::address::country_abr();
///
/// println!("Country (abbreviation): {}", country_abr);
/// ```
///
/// # Feature
///
/// Requires the "address" feature.
///
pub fn country_abr() -> String {
    misc::random_data(data::COUNTRY_ABR).to_string()
}

/// Generate a random latitude.
///
/// # Example
///
/// ```rust
/// use mockd::address::Info;
///
/// let latitude = mockd::address::latitude();
///
/// println!("Latitude: {}", latitude);
/// ```
///
/// # Feature
///
/// Requires the "address" feature.
///
pub fn latitude() -> f32 {
    misc::random::<f32>(-90.0, 90.0)
}

/// Generate a random latitude between a range of values.
///
/// # Example
///
/// ```rust
/// let latitude = mockd::address::latitude();
///
/// println!("Latitude: {}",latitude);
/// ```
///
/// # Error Handling
///
/// If the values are not within the range  -90.0 to 90.0 or the max value is not
/// greater than the min value the values are ignored and a random longitude is
/// returned.
///
/// # Feature
///
/// Requires the "address" feature.
///
pub fn latitude_in_range(min: f32, max: f32) -> f32 {
    if min > max || !(-90.0..=90.0).contains(&min) || !(-90.0..=90.0).contains(&max) {
        return latitude();
    }

    misc::random::<f32>(min, max)
}

/// Generate a random longitude.
///
/// # Example
///
/// ```rust
/// let longitude = mockd::address::longitude();
///
/// println!("Longitude: {}", longitude);
/// ```
///
/// # Feature
///
/// Requires the "address" feature.
///
pub fn longitude() -> f32 {
    misc::random::<f32>(-180.0, 180.0)
}

/// Generate a random longitude between a range of values.
///
/// # Example
///
/// ```rust
/// let longitude = mockd::address::longitude();
///
/// println!("Longitude: {}",longitude);
/// ```
///
/// # Error Handling
///
/// If the values are not within the range  -180.0 to 180.0 or the max value is not
/// greater than the min value the values are ignored and a random longitude is
/// returned.
///
/// # Feature
///
/// Requires the "address" feature.
///
pub fn longitude_in_range(min: f32, max: f32) -> f32 {
    if min > max || !(-180.0..=180.0).contains(&min) || {!(-180.0..=180.0).contains(&max) {
        return longitude();
    }

    misc::random::<f32>(min, max)
}

#[cfg(test)]
mod tests {
    use crate::address;
    use crate::testify::exec_mes;

    #[test]
    fn street() {
        exec_mes("address::street", address::street);
    }

    #[test]
    fn street_number() {
        exec_mes("address::street_number", address::street_number);
    }

    #[test]
    fn street_prefix() {
        exec_mes("address::street_prefix", address::street_prefix);
    }

    #[test]
    fn street_name() {
        exec_mes("address::street_name", address::street_name);
    }

    #[test]
    fn street_suffix() {
        exec_mes("address::street_suffix", address::street_suffix);
    }

    #[test]
    fn city() {
        exec_mes("address::city", address::city);
    }

    #[test]
    fn state() {
        exec_mes("address::state", address::state);
    }

    #[test]
    fn state_abr() {
        exec_mes("address::state_abr", address::state_abr);
    }

    #[test]
    fn zip() {
        exec_mes("address::zip", address::zip);
    }

    #[test]
    fn country() {
        exec_mes("address::country", address::country);
    }

    #[test]
    fn country_abr() {
        exec_mes("address::country_abr", address::country_abr);
    }

    #[test]
    fn latitude() {
        exec_mes("address::latitude", || format!("{}", address::latitude()));
    }

    #[test]
    fn latitude_in_range() {
        exec_mes("address::latitude_in_range", || {
            format!("{}", address::latitude_in_range(-30.0, 30.0))
        });
    }

    #[test]
    fn longitude() {
        exec_mes("address::longitude", || format!("{}", address::longitude()));
    }

    #[test]
    fn longitude_in_range() {
        exec_mes("address::longitude_in_range", || {
            format!("{}", address::longitude_in_range(-30.0, 30.0))
        });
    }
}
