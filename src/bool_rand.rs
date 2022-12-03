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

#[cfg(test)]
mod tests {
    use crate::misc;

    #[test]
    fn random_generation_for_boolean() {
        let mut count_0 = 0;
        let mut count_1 = 0;
        let mut count_other = 0;

        let mut i = 5;
        while i > 0 {
            let num = misc::random(0, 2);
            match num {
                0 => count_0 += 1,
                1 => count_1 += 1,
                _ => count_other += 1,
            }
            i -= 1;
        }

        println!(
            "\ncount_0:\t{}\ncount_1:\t{}\ncount_other:\t{}",
            count_0, count_1, count_other
        );

        assert_ne!(0, count_0);
        assert_ne!(0, count_1);
        assert_eq!(0, count_other);
    }

    #[test]
    fn bool_generation_is_random() {
        let mut true_count = 0;
        let mut false_count = 0;
        let mut i = 5;

        while i > 0 {
            if super::bool() {
                true_count += 1;
            } else {
                false_count += 1;
            }

            i -= 1;
        }

        println!("\ntrue:\t{}\nfalse:\t{}", true_count, false_count);

        assert_ne!(0, true_count);
        assert_ne!(0, false_count);
    }
}
