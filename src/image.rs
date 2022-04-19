//!
//! Provides 1 function to return random mock image.
//!
//! # Examples
//!
//! ##### image (1 function)
//!
//! ```rust
//! use mockd::image;
//!
//!     let data = image::url(500, 500); // url: https://picsum.photos/500/500
//! ```
//!

///
/// Generate a random mock image.
///
/// # inputs
///
/// * width - the width of the image
/// * height - the height of the image
///
/// # Examples
///
/// ##### image (1 function)
///
/// ```rust
///     let data = mockd::image::url(500, 500); // url: https://picsum.photos/500/500
///
/// ```
///
pub fn url(width: i64, height: i64) -> String {
    // url will generate a random Image Based Upon Height And Width. https://picsum.photos
    format!("https://picsum.photos/{}/{}", width, height)
}
