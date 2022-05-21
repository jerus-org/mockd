//!
//! Provides a function to return a random bool.
//!
//! # Examples
//!
//! ```rust
//! use mockd::bool_rand;
//!
//!     let data = bool_rand::bool(); // true / false
//!
//! ```
//!
//! # Feature
//!
//! Requires the "random-bool" feature.
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
/// # Feature
///
/// Requires the "random-bool" feature.
///
pub fn bool() -> bool {
    misc::random::<i64>(0, 1) == 1
}
