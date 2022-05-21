//!
//! Provides 8 functions to return mock beer data.
//!
//! # Example
//!
//! ```rust
//! use mockd::beer;
//!
//!     let data = beer::name(); // name: Sierra Nevada Bigfoot Barleywine Style Ale
//!     let data = beer::style(); // style: Porter
//!     let data = beer::hop(); // hop: Equinox
//!     let data = beer::yeast(); // yeast: 1084 - Irish Ale
//!     let data = beer::malt(); // malt: Roasted barley
//!     let data = beer::ibu(); // ibu: 75 IBU
//!     let data = beer::alcohol(); // alcohol: 2.943696 %
//!     let data = beer::blg(); // blg: 7.4607124°Blg
//! ```
//! # Feature
//!
//! Requires the "beer" feature.
//!

use crate::misc;

pub(crate) mod data;

/// Generate a random beer name.
///
/// # Example
///
/// ```rust
/// let beer = mockd::beer::name();
///
/// println!("Beer: {}", beer);
/// ```
///
/// # Feature
///
/// Requires the "beer" feature.
///
pub fn name() -> String {
    misc::random_data(data::NAME).to_string()
}

/// Generate a random beer style.
///
/// # Example
///
/// ```rust
/// let style = mockd::beer::style();
///
/// println!("Beer style: {}", style);
/// ```
///
/// # Feature
///
/// Requires the "beer" feature.
///
pub fn style() -> String {
    misc::random_data(data::STYLE).to_string()
}

/// Generate a random beer hop.
///
/// # Example
///
/// ```rust
/// let hop = mockd::beer::hop();
///
/// println!("Hop: {}", hop);
/// ```
///
/// # Feature
///
/// Requires the "beer" feature.
///
pub fn hop() -> String {
    misc::random_data(data::HOP).to_string()
}

/// Generate a random beer yeast.
///
/// # Example
///
/// ```rust
/// let yeast = mockd::beer::yeast();
///
/// println!("Beer Yeast: {}", yeast);
/// ```
///
/// # Feature
///
/// Requires the "beer" feature.
///
pub fn yeast() -> String {
    misc::random_data(data::YEAST).to_string()
}

/// Generate a random beer malt.
///
/// # Example
///
/// ```rust
/// let malt = mockd::beer::malt();
///
/// println!("Beer Hop: {}", malt);
/// ```
///
/// # Feature
///
/// Requires the "beer" feature.
///
pub fn malt() -> String {
    misc::random_data(data::MALT).to_string()
}

/// Generate a random beer ibu.
///
/// # Example
///
/// ```rust
/// let ibu = mockd::beer::ibu();
///
/// println!("Beer IBU: {}", ibu);
/// ```
///
/// # Feature
///
/// Requires the "beer" feature.
///
pub fn ibu() -> String {
    format!("{} IBU", misc::random::<i64>(10, 100))
}

/// Generate a random beer alcohol.
///
/// # Example
///
/// ```rust
/// let alcohol = mockd::beer::alcohol();
///
/// println!("Beer Alcohol: {}", alcohol);
/// ```
///
/// # Feature
///
/// Requires the "beer" feature.
///
pub fn alcohol() -> String {
    format!("{} %", misc::random::<f32>(2.0, 10.0))
}

/// Generate a random beer BLG.
///
/// # Example
///
/// ```rust
/// let blg = mockd::beer::blg();
///
/// println!("BLG: {}", blg);
/// ```
///
/// # Feature
///
/// Requires the "beer" feature.
///
pub fn blg() -> String {
    format!("{}°Blg", misc::random::<f32>(5.0, 20.0))
}

#[cfg(test)]
mod tests {
    use crate::beer;
    use crate::testify::exec_mes;

    #[test]
    fn name() {
        exec_mes("beer::name", beer::name);
    }

    #[test]
    fn style() {
        exec_mes("beer::style", beer::style);
    }

    #[test]
    fn hop() {
        exec_mes("beer::hop", beer::hop);
    }

    #[test]
    fn yeast() {
        exec_mes("beer::yeast", beer::yeast);
    }

    #[test]
    fn malt() {
        exec_mes("beer::malt", beer::malt);
    }

    #[test]
    fn ibu() {
        exec_mes("beer::ibu", beer::ibu);
    }

    #[test]
    fn alcohol() {
        exec_mes("beer::alcohol", beer::alcohol);
    }

    #[test]
    fn blg() {
        exec_mes("beer::blg", beer::blg);
    }
}
