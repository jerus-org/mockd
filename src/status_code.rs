//!
//! Provides 2 functions to return mock satus code data.
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

use crate::data::status_code;
use crate::misc;

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
pub fn simple() -> i16 {
    misc::random_data(status_code::SIMPLE)
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
pub fn general() -> i16 {
    misc::random_data(status_code::GENERAL)
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
