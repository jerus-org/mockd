//!
//! Provides 6 functions to return mock hacker data.
//!
//! # Examples
//!
//! ```rust
//! use mockd::hacker;
//!
//!     let data = hacker::phrase(); // phrase: parsing the sensor won't do anything, we need to bypass the open-source AGP sensor!
//!     let data = hacker::abbreviation(); // abbreviation: PCI
//!     let data = hacker::adjective(); // adjective: bluetooth
//!     let data = hacker::noun(); // noun: protocol
//!     let data = hacker::verb(); // verb: copy
//!     let data = hacker::ingverb(); // ingverb: transmitting
//! ```
//!
//! # Feature
//!
//! Requires the "hacker" feature.
//!

use crate::generator;
use crate::misc;

pub(crate) mod data;

/// Generate a random phrase.
///
/// # Example
///
/// ```rust
/// let phrase = mockd::hacker::phrase();
///
/// println!("Phrase: {}", phrase);
/// ```
///
/// # Feature
///
/// Requires the "hacker" feature.
///
pub fn phrase() -> String {
    let phrase = misc::random_data(data::PHRASE).to_string();
    generator::generate(phrase)
}

/// Pick a random abbreviation from the abbreviation dictionary.
///
/// # Example
///
/// ```rust
/// let abbreviation = mockd::hacker::abbreviation();
///
/// println!("Abbreviation: {}", abbreviation);
/// ```
///
/// # Feature
///
/// Requires the "hacker" feature.
///
pub fn abbreviation() -> String {
    misc::random_data(data::ABBREVIATION).to_string()
}

/// Pick a random adjective.from the adjective dictionary.
///
/// # Example
///
/// ```rust
/// let adjective = mockd::hacker::adjective();
///
/// println!("Adjective: {}", adjective);
/// ```
///
/// # Feature
///
/// Requires the "hacker" feature.
///
pub fn adjective() -> String {
    misc::random_data(data::ADJECTIVE).to_string()
}

/// Pick a random noun.from the noun dictionary.
///
/// # Example
///
/// ```rust
/// let noun = mockd::hacker::noun();
///
/// println!("Noun: {}", noun);
/// ```
///
/// # Feature
///
/// Requires the "hacker" feature.
///
pub fn noun() -> String {
    misc::random_data(data::NOUN).to_string()
}

/// Pick a random verb.from the verb dictionary.
///
/// # Example
///
/// ```rust
/// let verb = mockd::hacker::verb();
///
/// println!("Verb: {}", verb);
/// ```
///
/// # Feature
///
/// Requires the "hacker" feature.
///
pub fn verb() -> String {
    misc::random_data(data::VERB).to_string()
}

/// Pick a random ing verb form from the ingverb dictionary.
///
/// # Example
///
/// ```rust
/// let ingverb = mockd::hacker::ingverb();
///
/// println!("Ing form of verb: {}", ingverb);
/// ```
///
/// # Feature
///
/// Requires the "hacker" feature.
///
pub fn ingverb() -> String {
    misc::random_data(data::INGVERB).to_string()
}

#[cfg(test)]
mod tests {
    use crate::hacker;
    use crate::testify::exec_mes;

    #[test]
    fn phrase() {
        exec_mes("hacker::phrase", hacker::phrase);
    }

    #[test]
    fn abbreviation() {
        exec_mes("hacker::abbreviation", hacker::abbreviation);
    }

    #[test]
    fn adjective() {
        exec_mes("hacker::adjective", hacker::adjective);
    }

    #[test]
    fn noun() {
        exec_mes("hacker::noun", hacker::noun);
    }

    #[test]
    fn verb() {
        exec_mes("hacker::verb", hacker::verb);
    }

    #[test]
    fn ingverb() {
        exec_mes("hacker::ingverb", hacker::ingverb);
    }
}
