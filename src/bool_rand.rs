//!
//! Provides a function to return a random bool.
//!
//! # Examples
//!
//! ```rust
//! use mockd::bool;
//!
//!     let data = bool::bool(); // true / false
//!
use crate::misc;

/// Generate a random boolean value.
///
/// # Example
///
/// ```rust
/// let true_or_false = mockd::bool_rand::bool();
///
/// println!("True or False? {}", true_or_false);
/// ```
///
pub fn bool() -> bool {
    misc::random::<i64>(0, 1) == 1
}
