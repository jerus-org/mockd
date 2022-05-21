//!
//! Provides 2 functions to return mock unique data.
//!
//! # Examples
//!
//! ```rust
//! use mockd::unique;
//!
//!     let data = unique::uuid_v1(); // uuid_v1: 13be40a6-1dd2-11b2-802a-010203040506
//!     let data = unique::uuid_v4(); // uuid_v4: a474961e-936a-4897-966a-15fcff7bbc87
//! ```
//!
//! # Feature
//!
//! Requires the "unique" feature.
//!

use chrono::{Timelike, Utc};
use uuid::v1::{Context, Timestamp};
use uuid::Uuid;

/// Generate a v1 uuid.
///
/// # Example
///
/// ```rust
/// let uuid = mockd::unique::uuid_v1();
///
/// println!("Uuid: {}", uuid);
/// ```
///
/// # Feature
///
/// Requires the "unique" feature.
///
pub fn uuid_v1() -> String {
    let context = Context::new(42);
    let ts = Timestamp::from_unix(
        &context,
        Utc::now().second() as u64,
        Utc::now().timestamp_subsec_nanos(),
    );
    let uuid = Uuid::new_v1(ts, &[1, 2, 3, 4, 5, 6]);
    uuid.to_string()
}

/// Generate a v4 uuid.
///
/// # Example
///
/// ```rust
/// let uuid = mockd::unique::uuid_v4();
///
/// println!("Uuid: {}", uuid);
/// ```
///
/// # Feature
///
/// Requires the "unique" feature.
///
pub fn uuid_v4() -> String {
    Uuid::new_v4().to_string()
}

#[cfg(test)]
mod tests {
    use crate::testify::exec_mes;
    use crate::unique;

    #[test]
    fn uuid_v1() {
        exec_mes("unique::uuid_v1", unique::uuid_v1);
    }

    #[test]
    fn uuid_v4() {
        exec_mes("unique::uuid_v4", unique::uuid_v4);
    }
}
