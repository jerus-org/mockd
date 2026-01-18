//!
//! Provides 2 functions to return mock file data.
//!
//! # Examples
//!
//! ```rust
//! use mockd::file;
//!
//!     let data = file::mime_type(); // mime_type: text/x-fortran
//!     let data = file::extension(); // extension: aspx
//! ```
//!
//! # Feature
//!
//! Requires the "file" feature.
//!

use crate::misc;

pub(crate) mod data;

/// Generate a random mime type.
///
/// # Example
///
/// ```rust
/// let mime_type = mockd::file::mime_type();
///
/// println!("MIME Type: {}", mime_type);
/// ```
///
/// # Feature
///
/// Requires the "file" feature.
///
pub fn mime_type() -> String {
    misc::random_data(data::MIME_TYPE).to_string()
}

/// Generate a random file extension.
///
/// # Example
///
/// ```rust
/// let extension = mockd::file::extension();
///
/// println!("File extension: {}", extension);
/// ```
///
/// # Feature
///
/// Requires the "file" feature.
///
pub fn extension() -> String {
    misc::random_data(data::EXTENSION).to_string()
}

#[cfg(test)]
mod tests {
    use crate::file;
    use crate::testify::exec_mes;

    #[test]
    fn mime_type() {
        exec_mes("file::mime_type", file::mime_type);
    }

    #[test]
    fn extension() {
        exec_mes("file::extension", file::extension);
    }
}
