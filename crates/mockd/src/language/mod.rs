//!
//! Provides 3 functions to return mock language data.
//!
//! # Examples
//!
//! ```rust
//! use mockd::language;
//!
//!     let data = language::random(); // random: Tatar
//!     let data = language::abbreviation(); // abbreviation: co
//!     let data = language::programming(); // programming: Rust
//! ```
//!
//! # Feature
//!
//! Requires the "language" feature.
//!

use crate::misc;

pub(crate) mod data;

/// Pick a random spoken language from the language dictionary.
///
/// # Example
///
/// ```rust
/// let language = mockd::language::random();
///
/// println!("Language: {}", language);
/// ```
///
/// # Feature
///
/// Requires the "language" feature.
///
pub fn random() -> String {
    misc::random_data(data::LONG).to_string()
}

/// Pick a random spoken language short code from the language short code dictionary.
///
/// # Example
///
/// ```rust
/// let language_short_code = mockd::language::abbreviation ();
///
/// println!("Spoken language short code: {}", language_short_code);
/// ```
///
/// # Feature
///
/// Requires the "language" feature.
///
pub fn abbreviation() -> String {
    misc::random_data(data::SHORT).to_string()
}

/// Pick a random programming language from the programming language  dictionary.
///
/// # Example
///
/// ```rust
/// let programming_language = mockd::language::programming ();
///
/// println!("Programming language: {}", programming_language);
/// ```
///
/// # Feature
///
/// Requires the "language" feature.
///
pub fn programming() -> String {
    misc::random_data(data::PROGRAMMING).to_string()
}

#[cfg(test)]
mod tests {
    use crate::language;
    use crate::testify::exec_mes;

    #[test]
    fn random() {
        exec_mes("language::random", language::random);
    }

    #[test]
    fn abbreviation() {
        exec_mes("language::abbreviation", language::abbreviation);
    }

    #[test]
    fn programming() {
        exec_mes("language::programming", language::programming);
    }
}
