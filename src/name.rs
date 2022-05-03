//!
//! Provides 5 functions to return mock name data.
//!
//! # Examples
//!
//! ```rust
//! use mockd::name;
//!
//!     let data = name::full(); // full: Keyshawn Auer
//!     let data = name::first(); // first: Brycen
//!     let data = name::last(); // last: Hartmann
//!     let data = name::prefix(); // prefix: Mr.
//!     let data = name::suffix(); // suffix: PhD
//! ```
//!

use crate::misc;
use crate::person::data as person_data;
use ::std::string::String;

/// Generate a random full name by combining a random first and last name.
///
/// # Example
///
/// ```rust
/// let full_name = mockd::name::full();
///
/// println!("Full name: {}", full_name);
/// ```
///
pub fn full() -> String {
    format!("{} {}", first(), last())
}

/// Pick a random first name from the first name dictionary.
///
/// # Example
///
/// ```rust
/// let first_name = mockd::name::first();
///
/// println!("First name: {}", first_name);
/// ```
///
pub fn first() -> String {
    misc::random_data(person_data::FIRST).to_string()
}

/// Pick a random last name from the last name dictionary.
///
/// # Example
///
/// ```rust
/// let last_name = mockd::name::last();
///
/// println!("Last name: {}", last_name);
/// ```
///
pub fn last() -> String {
    misc::random_data(person_data::LAST).to_string()
}

/// Pick a random prefix from the prefix dictionary.
///
/// # Example
///
/// ```rust
/// let prefix = mockd::name::prefix();
///
/// println!("Name prefix: {}", prefix);
/// ```
///
pub fn prefix() -> String {
    misc::random_data(person_data::PREFIX).to_string()
}

/// Pick a random suffix from the suffix dictionary.
///
/// # Example
///
/// ```rust
/// let suffix = mockd::name::suffix();
///
/// println!("Name suffix: {}", suffix);
/// ```
///
pub fn suffix() -> String {
    misc::random_data(person_data::SUFFIX).to_string()
}

#[cfg(test)]
mod tests {
    use crate::name;
    use crate::testify::exec_mes;

    #[test]
    fn full() {
        exec_mes("name::full", name::full);
    }

    #[test]
    fn first() {
        exec_mes("name::first", name::first);
    }

    #[test]
    fn last() {
        exec_mes("name::last", name::last);
    }

    #[test]
    fn prefix() {
        exec_mes("name::prefix", name::prefix);
    }

    #[test]
    fn suffix() {
        exec_mes("name::suffix", name::suffix);
    }
}
