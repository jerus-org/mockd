//!
//! Provides 6 functions to return mock animal data.
//!
//! # Examples
//!
//! ```rust
//! use mockd::animal;
//!
//!     let data = animal::pet_name(); // pet_name: Squeakers
//!     let data = animal::animal(); // animal: salmon
//!     let data = animal::type_of(); // type_of: fish
//!     let data = animal::farm(); // farm: Sheep
//!     let data = animal::cat(); // cat: Oriental Shorthair
//!     let data = animal::dog(); // dog: Rottweiler
//! ```

use crate::data::animal;
use crate::misc;

/// Pick a random pet name from the petname dictionary.
///
/// # Example
///
/// ```rust
/// let pet_name = mockd::animal::pet_name();
///
/// println!("Pet Name: {}", pet_name);
/// ```
///
pub fn pet_name() -> String {
    misc::random_data(animal::PETNAME).to_string()
}

/// Pick a random animal from the animal dictionary.
///
/// # Example
///
/// ```rust
/// let animal = mockd::animal::animal();
///
/// println!("Animal: {}", animal);
/// ```
///
pub fn animal() -> String {
    misc::random_data(animal::ANIMAL).to_string()
}

/// Pick a random animal type from the animal types dictionary.
///
/// Valid types of animals:
/// * amphibians
/// * birds
/// * fis
/// * invertebrates
/// * mammals
/// * reptile
///
/// # Example
///
/// ```rust
/// let type_of_animal = mockd::animal::type_of();
///
/// println!("Type of Animal: {}", type_of_animal);
/// ```
///
pub fn type_of() -> String {
    misc::random_data(animal::TYPE).to_string()
}

/// Pick a random animal from the farm animals dictionary.
///
/// # Example
///
/// ```rust
/// let farm_animal = mockd::animal::farm();
///
/// println!("Farm Animal: {}", farm_animal);
/// ```
///
pub fn farm() -> String {
    misc::random_data(animal::FARM).to_string()
}

/// Pick a random cat breed from the cat breed dictionary.
///
/// # Example
///
/// ```rust
/// let cat = mockd::animal::cat();
///
/// println!("Breed of Cat: {}", cat);
/// ```
///
pub fn cat() -> String {
    misc::random_data(animal::CAT).to_string()
}

/// Pick a random dog breed from the dog breed dictionary.
///
/// # Example
///
/// ```rust
/// let dog = mockd::animal::dog();
///
/// println!("Breed of Dog: {}", dog);
/// ```
///
pub fn dog() -> String {
    misc::random_data(animal::DOG).to_string()
}

#[cfg(test)]
mod tests {
    use crate::animal;
    use crate::testify::exec_mes;

    #[test]
    fn pet_name() {
        exec_mes("animal::pet_name", animal::pet_name);
    }

    #[test]
    fn animal() {
        exec_mes("animal::animal", animal::animal);
    }

    #[test]
    fn type_of() {
        exec_mes("animal::type_of", animal::type_of);
    }

    #[test]
    fn farm() {
        exec_mes("animal::farm", animal::farm);
    }

    #[test]
    fn cat() {
        exec_mes("animal::cat", animal::cat);
    }

    #[test]
    fn dog() {
        exec_mes("animal::dog", animal::dog);
    }
}
