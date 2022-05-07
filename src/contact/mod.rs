//!
//! Provides 4 functions to return mock contact data.
//!
//! # Examples
//!
//! ```rust
//! use mockd::contact;
//!
//!     let data = contact::info(); // contact::Info
//!     let data = contact::phone(); // phone: 5173757868
//!     let data = contact::phone_formatted(); // phone_formatted: 382.450.6544
//!     let data = contact::email(); // email: benkuvalis@marks.org
//! ```
//! # Feature
//!
//! Requires the "contact" feature.
//!

use crate::internet::data as internet_data;
use crate::misc;
use crate::name;
use std::string::String;

pub(crate) mod data;

/// Info struct contains a phone number and email string.
/// # Example
///
/// ```rust
/// let info = mockd::contact::info();
///
/// println!("Info: {:#?}", info);
/// ```
///
/// # Feature
///
/// Requires the "contact" feature.
///
#[derive(Debug)]
pub struct Info {
    phone: String,
    email: String,
}

/// Generate a random info struct.
///
/// # Example
///
/// ```rust
/// let info = mockd::contact::info();
///
/// println!("Info: {:#?}", info);
/// ```
///
/// # Feature
///
/// Requires the "contact" feature.
///
pub fn info() -> Info {
    Info {
        phone: phone_formatted(),
        email: email(),
    }
}

/// Generate a random 11 digit phone number.
///
/// # Example
///
/// ```rust
/// let phone_number = mockd::contact::phone();
///
/// println!("Phone: {}", phone_number);
/// ```
///
/// # Feature
///
/// Requires the "contact" feature.
///
pub fn phone() -> String {
    misc::replace_with_numbers("##########".to_string())
}

/// Generate a random formatted phone number.
///
/// The phone number is formatted using a randomly selected format from:
/// * ###-###-####
/// * (###)###-####
/// * 1-###-###-####
/// * ###.###.####
///
/// # Example
///
/// ```rust
/// let formatted_phone = mockd::contact::phone_formatted();
///
/// println!("Formatted phone: {}", formatted_phone);
/// ```
///
/// # Feature
///
/// Requires the "contact" feature.
///
pub fn phone_formatted() -> String {
    misc::replace_with_numbers(misc::random_data(data::PHONE).to_string())
}

/// Generate a random email address.
///
/// # Example
///
/// ```rust
/// let email = mockd::contact::email();
///
/// println!("Email: {}", email);
/// ```
///
/// # Feature
///
/// Requires the "contact" feature.
///
pub fn email() -> String {
    format!(
        "{}{}@{}.{}",
        name::first(),
        name::last(),
        name::last(),
        misc::random_data(internet_data::DOMAIN_SUFFIX)
    )
    .to_lowercase()
}

#[cfg(test)]
mod tests {
    use crate::contact;
    use crate::testify::exec_mes;

    #[test]
    fn phone() {
        exec_mes("contact::phone", contact::phone);
    }

    #[test]
    fn phone_formatted() {
        exec_mes("contact::phone_formatted", contact::phone_formatted);
    }

    #[test]
    fn email() {
        exec_mes("contact::email", contact::email);
    }
}
