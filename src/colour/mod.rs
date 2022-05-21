//!
//! Provides 4 functions to return mock colour data.
//!
//! # Examples
//!
//! ```rust
//! use mockd::colour;
//!
//!     let data = colour::full(); // full: LightYellow
//!     let data = colour::hex(); // hex: #662461
//!     let data = colour::safe(); // safe: black
//!     let data = colour::rgb(); // rgb: [162, 98, 22]
//! ```
//!
//! # Feature
//!
//! Requires the "colour" feature.
//!

use crate::misc;
pub(crate) mod data;

/// Generate a random colour name.
///
/// # Example
///
/// ```rust
/// let colour = mockd::colour::full();
///
/// println!("Color name from full colour name list: {}", colour);
/// ```
///
/// # Feature
///
/// Requires the "colour" feature.
///
pub fn full() -> String {
    misc::random_data(data::FULL).to_string()
}

/// Generate a random colour hex code.
///
/// # Example
///
/// ```rust
/// let hex = mockd::colour::hex();
///
/// println!("Color hex: {}", hex);
/// ```
///
/// # Feature
///
/// Requires the "colour" feature.
///
pub fn hex() -> String {
    let mut rand: [&'static str; 6] = [
        misc::HASHTAG,
        misc::HASHTAG,
        misc::HASHTAG,
        misc::HASHTAG,
        misc::HASHTAG,
        misc::HASHTAG,
    ];

    #[allow(clippy::needless_range_loop)]
    for x in 0..5 {
        match misc::random::<i8>(0, 1) {
            0 => rand[x] = misc::HASHTAG,
            1 => rand[x] = misc::QUESTIONMARK,
            _ => println!("impossible"),
        }
    }

    format!(
        "#{}",
        misc::replace_with_letter_hex(misc::replace_with_numbers(rand.join("")))
    )
}

/// Generate a random safe colour.
///
/// # Example
///
/// ```rust
/// let safe_colour = mockd::colour::safe();
///
/// println!("Safe colour: {}", safe_colour);
/// ```
///
/// # Feature
///
/// Requires the "colour" feature.
///
pub fn safe() -> String {
    misc::random_data(data::SAFE).to_string()
}

/// Generate an array of three random rgb numbers.
///
/// # Example
///
/// ```rust
/// let rgb = mockd::colour::rgb();
///
/// println!("Red: {}", rgb[0]);
/// println!("Green: {}", rgb[1]);
/// println!("Blue: {}", rgb[2]);
/// ```
///
/// # Feature
///
/// Requires the "colour" feature.
///
pub fn rgb() -> [i16; 3] {
    [
        misc::random::<i16>(0, 255),
        misc::random::<i16>(0, 255),
        misc::random::<i16>(0, 255),
    ]
}

#[cfg(test)]
mod tests {
    use crate::colour;
    use crate::testify::exec_mes;

    #[test]
    fn full() {
        exec_mes("colour::full", colour::full);
    }

    #[test]
    fn hex() {
        exec_mes("colour::hex", colour::hex);
    }

    #[test]
    fn safe() {
        exec_mes("colour::safe", colour::safe);
    }

    #[test]
    fn rgb() {
        exec_mes("colour::rgb", || format!("{:?}", colour::rgb()));
    }
}
