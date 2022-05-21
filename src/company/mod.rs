//!
//! Provides 6 functions to return mock animal data.
//!
//! # Examples
//!
//! ```rust
//! use mockd::company;
//!
//!     let data = company::company(); // company: Rowe-Schoen
//!     let data = company::company_suffix(); // company_suffix: Inc
//!     let data = company::buzzword(); // buzzword: systemic
//!     let data = company::bs(); // bs: strategic
//! ```
//!
//! # Feature
//!
//! Requires the "company" feature.
//!

use crate::misc;
use crate::name;
use std::string::String;

pub(crate) mod data;

/// Generate a random company name.
///
/// # Example
///
/// ```rust
/// let company = mockd::company::company();
///
/// println!("Company name: {}", company);
/// ```
///
/// # Feature
///
/// Requires the "company" feature.
///
pub fn company() -> String {
    match misc::random::<i64>(1, 3) {
        1 => return format!("{}, {} and {}", name::last(), name::last(), name::last()),
        2 => return format!("{}-{}", name::last(), name::last()),
        3 => return format!("{} {}", name::last(), company_suffix()),
        _ => "impossible".to_string(),
    }
}

/// Generate a random company suffix.
///
/// # Example
///
/// ```rust
/// let company_suffix = mockd::company::company_suffix();
///
/// println!("Company suffix: {}", company_suffix);
/// ```
///
/// # Feature
///
/// Requires the "company" feature.
///
pub fn company_suffix() -> String {
    misc::random_data(data::SUFFIX).to_string()
}

/// Generate a random company buzzword.
///
/// # Example
///
/// ```rust
/// let buzzword = mockd::company::buzzword();
///
/// println!("Company buzzword: {}", buzzword);
/// ```
///
/// # Feature
///
/// Requires the "company" feature.
///
pub fn buzzword() -> String {
    misc::random_data(data::BUZZWORDS).to_string()
}

/// Generate a random company bs.
///
/// # Example
///
/// ```rust
/// let bs = mockd::company::bs();
///
/// println!("Company bs: {}", bs);
/// ```
///

/// # Feature
///
/// Requires the "company" feature.
///
pub fn bs() -> String {
    misc::random_data(data::BS).to_string()
}

#[cfg(test)]
mod tests {
    use crate::company;
    use crate::testify::exec_mes;

    #[test]
    fn company() {
        exec_mes("company::company", company::company);
    }

    #[test]
    fn company_suffix() {
        exec_mes("company::company_suffix", company::company_suffix);
    }

    #[test]
    fn buzzword() {
        exec_mes("company::buzzword", company::buzzword);
    }

    #[test]
    fn bs() {
        exec_mes("company::bs", company::bs);
    }
}
