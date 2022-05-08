//!
//! Provides 6 functions to return mock vehicle data.
//!
//! # Examples
//!
//! ```rust
//! use mockd::vehicle;
//!
//!     let data = vehicle::info(); // vehicle::Info
//!     let data = vehicle::vehicle_type(); // vehicle_type: Passenger car mini
//!     let data = vehicle::fuel(); // fuel: Electric
//!     let data = vehicle::transmission_gear(); // transmission_gear: Automatic
//!     let data = vehicle::car_maker(); // car_maker: Chevrolet
//!     let data = vehicle::car_model(); // car_model: Gti
//! ```
//!
//! # Feature
//!
//! Requires the "vehicle" feature.
//!

use crate::misc;
use chrono::{Datelike, Utc};

pub(crate) mod data;

/// Info struct for vehicle data.
///
/// # Example
///
/// ```rust
/// let vehicle = mockd::vehicle::info();
///
/// println!("Vehicle: {:#?}", vehicle);
/// ```
///
/// # Feature
///
/// Requires the "vehicle" feature.
///
#[derive(Debug)]
pub struct Info {
    vehicle_type: String,
    fuel: String,
    transmission_gear: String,
    brand: String,
    model: String,
    year: i32,
}

/// Generate a random info struct of vehicle data.
///
/// # Example
///
/// ```rust
/// let vehicle = mockd::vehicle::info();
///
/// println!("Vehicle: {:#?}", vehicle);
/// ```
///
/// # Feature
///
/// Requires the "vehicle" feature.
///
pub fn info() -> Info {
    Info {
        vehicle_type: vehicle_type(),
        fuel: fuel(),
        transmission_gear: transmission_gear(),
        brand: car_maker(),
        model: car_model(),
        year: misc::random::<i32>(0, Utc::now().year()),
    }
}

/// Pick a random vehicle type from the type dictionary.
///
/// # Example
///
/// ```rust
/// let vehicle_type = mockd::vehicle::vehicle_type();
///
/// println!("Vehicle type: {}", vehicle_type);
/// ```
///
/// # Feature
///
/// Requires the "vehicle" feature.
///
pub fn vehicle_type() -> String {
    misc::random_data(data::TYPE).to_string()
}

/// Pick a random fuel type from the fuel type dictionary.
///
/// # Example
///
/// ```rust
/// let fuel_type = mockd::vehicle::fuel();
///
/// println!("Fuel: {}", fuel_type);
/// ```
///
/// # Feature
///
/// Requires the "vehicle" feature.
///
pub fn fuel() -> String {
    misc::random_data(data::FUEL_TYPE).to_string()
}

/// Pick a random transmission from the transmission dictionary.
///
/// # Example
///
/// ```rust
/// let transmission = mockd::vehicle::transmission_gear();
///
/// println!("Transmission: {}", transmission);
/// ```
///
/// # Feature
///
/// Requires the "vehicle" feature.
///
pub fn transmission_gear() -> String {
    misc::random_data(data::TRANSMISSION_TYPE).to_string()
}

/// Pick a random car maker from the car maker dictionary.
///
/// # Example
///
/// ```rust
/// let maker = mockd::vehicle::car_maker();
///
/// println!("Maker: {}", maker);
/// ```
///
/// # Feature
///
/// Requires the "vehicle" feature.
///
pub fn car_maker() -> String {
    misc::random_data(data::MAKER).to_string()
}

/// Pick a random model from the model dictionary.
///
/// # Example
///
/// ```rust
/// let model = mockd::vehicle::car_model();
///
/// println!("Model: {}", model);
/// ```
///
/// # Feature
///
/// Requires the "vehicle" feature.
///
pub fn car_model() -> String {
    misc::random_data(data::MODEL).to_string()
}

#[cfg(test)]
mod tests {
    use crate::testify::exec_mes;
    use crate::vehicle;

    #[test]
    fn vehicle_type() {
        exec_mes("vehicle::vehicle_type", vehicle::vehicle_type);
    }

    #[test]
    fn fuel() {
        exec_mes("vehicle::fuel", vehicle::fuel);
    }

    #[test]
    fn transmission_gear() {
        exec_mes("vehicle::transmission_gear", || {
            vehicle::transmission_gear()
        });
    }

    #[test]
    fn car_maker() {
        exec_mes("vehicle::car_maker", vehicle::car_maker);
    }

    #[test]
    fn car_model() {
        exec_mes("vehicle::car_model", vehicle::car_model);
    }
}
