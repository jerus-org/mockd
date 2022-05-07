//!
//! Provides 4 functions to return mock job data.
//!
//! # Examples
//!
//! ```rust
//! use mockd::job;
//!
//!     let data = job::info(); // job::Info
//!     let data = job::title(); // title: Executive
//!     let data = job::descriptor(); // descriptor: International
//!     let data = job::level(); // level: Solutions
//! ```
//!
//! # Feature
//!
//! Requires the "job" feature.
//!

use crate::company;
use crate::misc;

pub(crate) mod data;

/// Job struct consisting of:
/// * company
/// * title
/// * descriptor
/// * level
///
/// # Example
///
/// ```rust
/// let job_info = mockd::job::info();
///
/// println!("Job Info: {:#?}", job_info);
/// ```
///
/// # Feature
///
/// Requires the "job" feature.
///
#[derive(Debug)]
pub struct Info {
    company: String,
    title: String,
    descriptor: String,
    level: String,
}

/// Generate a random job info struct .
///
/// # Example
///
/// ```rust
/// let job_info = mockd::job::info();
///
/// println!("Job Info: {:#?}", job_info);
/// ```
///
/// # Feature
///
/// Requires the "job" feature.
///
pub fn info() -> Info {
    Info {
        company: company::company(),
        title: title(),
        descriptor: descriptor(),
        level: level(),
    }
}

/// Pick a random job title from the title dictionary.
///
/// # Example
///
/// ```rust
/// let title = mockd::job::title();
///
/// println!("Title: {}", title);
/// ```
///
/// # Feature
///
/// Requires the "job" feature.
///
pub fn title() -> String {
    misc::random_data(data::TITLE).to_string()
}

/// Pick a random job descriptor from the descriptor dictionary.
///
/// # Example
///
/// ```rust
/// let descriptor = mockd::job::descriptor();
///
/// println!("Descriptor: {}", descriptor);
/// ```
///
/// # Feature
///
/// Requires the "job" feature.
///
pub fn descriptor() -> String {
    misc::random_data(data::DESCRIPTOR).to_string()
}

/// Pick a random job level from the level dictionary.
///
/// # Example
///
/// ```rust
/// let level = mockd::job::level();
///
/// println!("Level: {}", level);
/// ```
///
/// # Feature
///
/// Requires the "job" feature.
///
pub fn level() -> String {
    misc::random_data(data::LEVEL).to_string()
}

#[cfg(test)]
mod tests {
    use crate::job;
    use crate::testify::exec_mes;

    #[test]
    fn title() {
        exec_mes("job::title", job::title);
    }

    #[test]
    fn descriptor() {
        exec_mes("job::descriptor", job::descriptor);
    }

    #[test]
    fn level() {
        exec_mes("job::level", job::level);
    }
}
