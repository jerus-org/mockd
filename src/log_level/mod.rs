//!
//! Provides 3 functions to return mock log level data.
//!
//! # Examples
//!
//! ```rust
//! use mockd::log_level;
//!
//!     let data = log_level::general(); // general: info
//!     let data = log_level::syslog(); // syslog: crit
//!     let data = log_level::apache(); // apache: debug
//! ```
//!
//! # Feature
//!
//! Requires the "log-level" feature.
//!

use crate::misc;

pub(crate) mod data;

/// Pick a random log level from the log level dictionary.
///
/// # Example
///
/// ```rust
/// let log_level = mockd::log_level::general();
///
/// println!("Log level: {}", log_level);
/// ```
///
/// # Feature
///
/// Requires the "log-level" feature.
///
pub fn general() -> String {
    misc::random_data(data::GENERAL).to_string()
}

/// Pick a random syslog log level from the syslog dictionary.
///
/// # Example
///
/// ```rust
/// let log_level = mockd::log_level::syslog();
///
/// println!("SysLog level: {}", log_level);
/// ```
///
/// # Feature
///
/// Requires the "log-level" feature.
///
pub fn syslog() -> String {
    misc::random_data(data::SYSLOG).to_string()
}

/// Pick a random Apache log level from the Apache log level dictionary.
///
/// # Example
///
/// ```rust
/// let apache_log_level = mockd::log_level::apache();
///
/// println!("Apache log level: {}", apache_log_level);
/// ```
///
/// # Feature
///
/// Requires the "log-level" feature.
///
pub fn apache() -> String {
    misc::random_data(data::APACHE).to_string()
}

#[cfg(test)]
mod tests {
    use crate::log_level;
    use crate::testify::exec_mes;

    #[test]
    fn general() {
        exec_mes("log_level::general", log_level::general);
    }

    #[test]
    fn apache() {
        exec_mes("log_level::apache", log_level::apache);
    }

    #[test]
    fn syslog() {
        exec_mes("log_level::syslog", log_level::syslog);
    }
}
