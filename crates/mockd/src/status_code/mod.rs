//!
//! Provides 2 functions to return mock status code data.
//!
//! # Examples
//!
//! ```rust
//!use mockd::status_code;
//!
//!    let data = status_code::simple(); // simple: 404
//!    let data = status_code::general(); // general: 400
//!```
//!
//! # Feature
//!
//! Requires the "status-code" feature.
//!

use crate::misc;

pub(crate) mod data;

/// Pick a random status code from the simple dictionary.
///
/// # Example
///
/// ```rust
/// let simple = mockd::status_code::simple();
///
/// println!("Simple status code: {}", simple);
/// ```
///
/// # Feature
///
/// Requires the "status-code" feature.
///
pub fn simple() -> i16 {
    misc::random_data(data::SIMPLE)
}

/// Pick a random status code from the general dictionary.
///
/// # Example
///
/// ```rust
/// let general = mockd::status_code::general();
///
/// println!("General status code: {}", general);
/// ```
///
/// # Feature
///
/// Requires the "status-code" feature.
///
pub fn general() -> i16 {
    misc::random_data(data::GENERAL)
}

#[cfg(test)]
mod tests {
    use crate::status_code;
    use crate::testify::exec_mes;

    #[test]
    fn simple() {
        exec_mes("status_code::simple", || {
            format!("{}", status_code::simple())
        });
    }

    #[test]
    fn general() {
        exec_mes("status_code::general", || {
            format!("{}", status_code::general())
        });
    }
}
