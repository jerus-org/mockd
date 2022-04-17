//!
//! Provides 4 functions to return mock color data.
//!
//! # Examples
//!
//! ```rust
//! use mockd::color;
//!
//!     let data = color::full(); // full: LightYellow
//!     let data = color::hex(); // hex: #662461
//!     let data = color::safe(); // safe: black
//!     let data = color::rgb(); // rgb: [162, 98, 22]
//! ```
//!

use crate::data::color;
use crate::misc;

/// Generate a random color name.
///
/// # Example
///
/// ```rust
/// let color = mockd::color::full();
///
/// println!("Color name from full color name list: {}", color);
/// ```
///
pub fn full() -> String {
    misc::random_data(color::FULL).to_string()
}

/// Generate a random color hex code.
///
/// # Example
///
/// ```rust
/// let hex = mockd::color::hex();
///
/// println!("Color hex: {}", hex);
/// ```
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

/// Generate a random safe color.
///
/// # Example
///
/// ```rust
/// let safe_color = mockd::color::safe();
///
/// println!("Safe color: {}", safe_color);
/// ```
///
pub fn safe() -> String {
    misc::random_data(color::SAFE).to_string()
}

/// Generate an array of three random rgb numbers.
///
/// # Example
///
/// ```rust
/// let rgb = mockd::color::rgb();
///
/// println!("Red: {}", rgb[0]);
/// println!("Green: {}", rgb[1]);
/// println!("Blue: {}", rgb[2]);
/// ```
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
    use crate::color;
    use crate::testify::exec_mes;

    #[test]
    fn full() {
        exec_mes("color::full", color::full);
    }

    #[test]
    fn hex() {
        exec_mes("color::hex", color::hex);
    }

    #[test]
    fn safe() {
        exec_mes("color::safe", color::safe);
    }

    #[test]
    fn rgb() {
        exec_mes("color::rgb", || format!("{:?}", color::rgb()));
    }
}
