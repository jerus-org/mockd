//!
//! Provides 4 functions to return currency data.
//!
//! # Examples
//!
//! ```rust
//! use mockd::currency;
//!
//!     let data = currency::compact(); // currency::Info
//!     let data = currency::short(); // short: SRD
//!     let data = currency::long(); // long: Burundi Franc
//!     let data = currency::price(1 as f64, 123 as f64); // price: 53.7
//! ```

use crate::data::currency;
use crate::misc;
use math::round;

/// Contains both the short and long name for a currency
#[derive(Debug)]
pub struct Info {
    short: String,
    long: String,
}

/// Generate a random currency description based on info struct.
///
/// # Example
///
/// ```rust
/// let compact = mockd::currency::compact();
///
/// println!("Currency: {:#?}", compact);
/// ```
///
pub fn compact() -> Info {
    let index = misc::random_data_index(currency::SHORT);
    Info {
        short: currency::SHORT[index].to_string(),
        long: currency::LONG[index].to_string(),
    }
}

/// Generate a random short currency code.
///
/// # Example
///
/// ```rust
/// let short = mockd::currency::short();
///
/// println!("Currency code: {}", short);
/// ```
///
pub fn short() -> String {
    misc::random_data(currency::SHORT).to_string()
}

/// Generate a random long currency name.
///
/// # Example
///
/// ```rust
/// let name = mockd::currency::long();
///
/// println!("Currency name: {}", name);
/// ```
///
pub fn long() -> String {
    misc::random_data(currency::LONG).to_string()
}

/// Generate a random currency price within a range.
///
/// Given a range provide a currency price to two places of decimal.
///
/// # Example
///
/// ```rust
/// let price = mockd::currency::price(10.0,39.5);
///
/// println!("Currency price: {}", price);
/// ```
///
pub fn price(min: f64, max: f64) -> f64 {
    round::floor(misc::random::<f64>(min, max), 2)
}

#[cfg(test)]
mod tests {
    use crate::currency;
    use crate::testify::exec_mes;

    #[test]
    fn short() {
        exec_mes("currency::short", currency::short);
    }

    #[test]
    fn long() {
        exec_mes("currency::long", currency::long);
    }

    #[test]
    fn price() {
        exec_mes("currency::short", || {
            format!("{}", currency::price(1.0_f64, 123.0_f64))
        });
    }
}
