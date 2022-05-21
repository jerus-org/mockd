//!
//! Provides 8 functions to return mock user agent data.
//!
//! # Examples
//!
//! ```rust
//! use mockd::user_agent;
//!
//!     let data = user_agent::chrome(); // chrome: Mozilla/5.0 (X11; Linux i686) AppleWebKit/532 (KHTML, like Gecko) Chrome/36.0.861.0 Mobile Safari/532
//!     let data = user_agent::firefox(); // firefox: Mozilla/5.0 (X11; Linux x86_64; rv:7.0) Gecko/2005-5-27 Firefox/36.0
//!     let data = user_agent::safari(); // safari: Mozilla/5.0 (Windows; U; Windows NT 6.2) AppleWebKit/531.23.3 (KHTML, like Gecko) Version/4.0 Safari/531.23.3
//!     let data = user_agent::opera(); // opera: Opera/8.22 (Macintosh; PPC Mac OS X 10_6_8; en-US) Presto/2.11.181 Version/12.00
//!     let data = user_agent::linux_platform_token(); // linux_platform_token: X11; Linux x86_64
//!     let data = user_agent::mac_platform_token(); // mac_platform_token: Macintosh; U; PPC Mac OS X 10_6_2
//!     let data = user_agent::windows_platform_token(); // windows_platform_token: Windows 98; Win 9x 4.90
//!     let data = user_agent::random_platform(); // random_platform: Macintosh; Intel Mac OS X 10_7_5
//! ```
//!
//! # Feature
//!
//! Requires the "user-agent" feature.
//!

use crate::datetime;
use crate::misc;

pub(crate) mod data;

/// Generate a random Chrome user agent string.
///
/// # Example
///
/// ```rust
/// let user_agent = mockd::user_agent::chrome();
///
/// println!("User agent string: {}", user_agent);
/// ```
///
/// # Feature
///
/// Requires the "user-agent" feature.
///
pub fn chrome() -> String {
    let rand_num = misc::random(531, 536) + misc::random(0, 2);
    format!(
        "Mozilla/5.0 ({}) AppleWebKit/{} (KHTML, like Gecko) Chrome/{}.0.{}.0 Mobile Safari/{}",
        random_platform(),
        rand_num,
        misc::random(36, 40),
        misc::random(800, 899),
        rand_num
    )
}

/// Generate a random Firefox user agent string.
///
/// # Example
///
/// ```rust
/// let user_agent = mockd::user_agent::firefox();
///
/// println!("User agent string: {}", user_agent);
/// ```
///
/// # Feature
///
/// Requires the "user-agent" feature.
///
pub fn firefox() -> String {
    // @TODO should be 2006-02-01
    let date = format!(
        "{}-{}-{}",
        datetime::year(),
        datetime::month(),
        datetime::day()
    );
    let platform = match misc::random(1, 3) {
        1 => format!(
            "({}; en-US; rv:1.9.{}.20)",
            windows_platform_token(),
            misc::random(0, 3)
        ),
        2 => format!("({}; rv:{}.0)", linux_platform_token(), misc::random(5, 8)),
        _ => format!("({} rv:{}.0)", mac_platform_token(), misc::random(2, 7)),
    };
    format!(
        "Mozilla/5.0 {} Gecko/{} Firefox/{}.0",
        platform,
        date,
        misc::random(35, 37)
    )
}

/// Generate a random Safari user agent string.
///
/// # Example
///
/// ```rust
/// let user_agent = mockd::user_agent::safari();
///
/// println!("User agent string: {}", user_agent);
/// ```
///
/// # Feature
///
/// Requires the "user-agent" feature.
///
pub fn safari() -> String {
    let rand_num = format!(
        "{}.{}.{}",
        misc::random(531, 536),
        misc::random(1, 51),
        misc::random(1, 8),
    );

    let ver = format!("{}.{}", misc::random(4, 6), misc::random(0, 2));

    let mobile_devices = match misc::random(1, 2) {
        1 => String::from("iPhone; CPU iPhone OS"),
        _ => String::from("iPad; CPU OS"),
    };

    let platforms = match misc::random(1,3) {
        1 => format!("(Windows; U; {}) AppleWebKit/{} (KHTML, like Gecko) Version/{} Safari/{}", windows_platform_token(), rand_num, ver, rand_num),
        2 => format!("({} rv:{}.0; en-US) AppleWebKit/{} (KHTML, like Gecko) Version/{} Safari/{}", mac_platform_token(), misc::random(4, 7), rand_num, ver, rand_num),
        _ => format!("({} {}_{}_{} like Mac OS X; en-US) AppleWebKit/{} (KHTML, like Gecko) Version/{}.0.5 Mobile/8B{} Safari/6{}", mobile_devices, misc::random(7, 9 ), misc::random(0, 3), misc::random(1, 3), rand_num, misc::random(3, 5), misc::random(111, 120), rand_num)
    };

    format!("Mozilla/5.0 {}", platforms)
}

/// Generate a random Opera user agent string.
///
/// # Example
///
/// ```rust
/// let user_agent = mockd::user_agent::opera();
///
/// println!("User agent string: {}", user_agent);
/// ```
///
/// # Feature
///
/// Requires the "user-agent" feature.
///
pub fn opera() -> String {
    let platform = format!(
        "({}; en-US) Presto/2.{}.{} Version/{}.00",
        random_platform(),
        misc::random(8, 13),
        misc::random(160, 355),
        misc::random(10, 13)
    );

    format!(
        "Opera/{}.{} {}",
        misc::random(8, 10),
        misc::random(10, 99),
        platform
    )
}

/// Generate a random Linux platform token string.
///
/// # Example
///
/// ```rust
/// let platform_token = mockd::user_agent::linux_platform_token();
///
/// println!("Platform token: {}", platform_token);
/// ```
///
/// # Feature
///
/// Requires the "user-agent" feature.
///
pub fn linux_platform_token() -> String {
    format!("X11; Linux {}", misc::random_data(data::LINUX_PROCESSOR))
}

/// Generate a random Mac platform token string.
///
/// # Example
///
/// ```rust
/// let platform_token = mockd::user_agent::mac_platform_token();
///
/// println!("Platform token: {}", platform_token);
/// ```
///
/// # Feature
///
/// Requires the "user-agent" feature.
///
pub fn mac_platform_token() -> String {
    format!(
        "Macintosh; {} Mac OS X 10_{}_{}",
        misc::random_data(data::MAC_PROCESSOR),
        misc::random(5, 9),
        misc::random(0, 10),
    )
}

/// Generate a random Windows platform token string.
///
/// # Example
///
/// ```rust
/// let platform_token = mockd::user_agent::windows_platform_token();
///
/// println!("Platform token: {}", platform_token);
/// ```
///
/// # Feature
///
/// Requires the "user-agent" feature.
///
pub fn windows_platform_token() -> String {
    misc::random_data(data::WINDOWS_PLATFORM).to_string()
}

/// Generate a random platform token string.
///
/// # Example
///
/// ```rust
/// let platform_token = mockd::user_agent::random_platform();
///
/// println!("Platform token: {}", platform_token);
/// ```
///
/// # Feature
///
/// Requires the "user-agent" feature.
///
pub fn random_platform() -> String {
    match misc::random(1, 3) {
        1 => linux_platform_token(),
        2 => mac_platform_token(),
        _ => windows_platform_token(),
    }
}

#[cfg(test)]
mod tests {
    use crate::testify::exec_mes;
    use crate::user_agent;

    #[test]
    fn chrome() {
        exec_mes("user_agent::chrome", user_agent::chrome);
    }

    #[test]
    fn firefox() {
        exec_mes("user_agent::firefox", user_agent::firefox);
    }

    #[test]
    fn safari() {
        exec_mes("user_agent::safari", user_agent::safari);
    }

    #[test]
    fn opera() {
        exec_mes("user_agent::opera", user_agent::opera);
    }

    #[test]
    fn linux_platform_token() {
        exec_mes("user_agent::linux_platform_token", || {
            user_agent::linux_platform_token()
        });
    }

    #[test]
    fn mac_platform_token() {
        exec_mes("user_agent::mac_platform_token", || {
            user_agent::mac_platform_token()
        });
    }

    #[test]
    fn windows_platform_token() {
        exec_mes("user_agent::windows_platform_token", || {
            user_agent::windows_platform_token()
        });
    }

    #[test]
    fn random_platform() {
        exec_mes("user_agent::random_platform", || {
            user_agent::random_platform()
        });
    }
}
