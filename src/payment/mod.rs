//!
//! Provides 5 functions to return mock payment data.
//!
//! # Examples
//!
//! ```rust
//! use mockd::payment;
//!
//!     let data = payment::credit_card(); // payment::CreditCard
//!     let data = payment::credit_card_type(); // credit_card_type: Discover
//!     let data = payment::credit_card_number(); // credit_card_number: 341545247171534
//!     let data = payment::credit_card_exp(); // credit_card_exp: 04/21
//!     let data = payment::credit_card_cvv(); // credit_card_cvv: 537
//! ```
//!

use crate::misc;
use chrono::{Datelike, Utc};

mod data;

/// Struct to describe a credit card.
///
/// #Example
///
/// ```rust
/// let credit_card= mockd::payment::credit_card();
///
/// println!("Credit Card: {:#?}", credit_card);
/// ```
#[derive(Debug)]
pub struct CreditCard {
    type_of: String,
    number: String,
    exp: String,
    cvv: String,
}

/// Generate a random credit card data set.
///
/// # Example
///
/// ```rust
/// let credit_card= mockd::payment::credit_card();
///
/// println!("Credit Card: {:#?}", credit_card);
/// ```
///
pub fn credit_card() -> CreditCard {
    CreditCard {
        type_of: credit_card_type(),
        number: credit_card_number(),
        exp: credit_card_exp(),
        cvv: credit_card_cvv(),
    }
}

/// Pick a random credit card type from the type dictionary.
///
/// # Example
///
/// ```rust
/// let card_type = mockd::payment::credit_card_type();
///
/// println!("Credit card type: {}", card_type);
/// ```
///
pub fn credit_card_type() -> String {
    misc::random_data(data::CARD_TYPE).to_string()
}

/// Generate a random credit card number.
///
/// # Example
///
/// ```rust
/// let card_number = mockd::payment::credit_card_number();
///
/// println!("Credit card number: {}", card_number);
/// ```
///
pub fn credit_card_number() -> String {
    misc::replace_with_numbers(misc::random_data(data::NUMBER).to_string())
}

/// Pick a random credit card type from the type dictionary.
///
/// # Example
///
/// ```rust
/// let card_type = mockd::payment::credit_card_type();
///
/// println!("Credit card type: {}", card_type);
/// ```
///
fn credit_card_luhn_number() -> String {
    // @TODO
    String::from("")
}

// Generate a random credit card expiry date.
///
/// # Example
///
/// ```rust
/// let card_exp = mockd::payment::credit_card_exp();
///
/// println!("Credit card expiry date: {}", card_exp);
/// ```
///
pub fn credit_card_exp() -> String {
    let current_year = Utc::now().year() - 2000;
    let month = misc::random(1, 12);
    if month < 10 {
        format!(
            "{}/{}",
            format!("0{}", month).as_str(),
            current_year + misc::random(1, 10)
        )
    } else {
        format!(
            "{}/{}",
            format!("{}", month).as_str(),
            current_year + misc::random(1, 10)
        )
    }
}

/// Generate a random credit card cvv.
///
/// # Example
///
/// ```rust
/// let card_ccv = mockd::payment::credit_card_cvv();
///
/// println!("Credit card ccv: {}", card_ccv);
/// ```
///
pub fn credit_card_cvv() -> String {
    misc::replace_with_numbers("###".to_string())
}

#[cfg(test)]
mod tests {
    use crate::payment;
    use crate::testify::exec_mes;

    #[test]
    fn credit_card_type() {
        exec_mes("payment::credit_card_type", payment::credit_card_type);
    }

    #[test]
    fn credit_card_number() {
        exec_mes("payment::credit_card_number", || {
            payment::credit_card_number()
        });
    }

    #[test]
    fn credit_card_exp() {
        exec_mes("payment::credit_card_exp", payment::credit_card_exp);
    }

    #[test]
    fn credit_card_cvv() {
        exec_mes("payment::credit_card_cvv", payment::credit_card_cvv);
    }
}
