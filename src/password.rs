//!
//! Provides 1 function to return mock password data.
//!
//! # Examples
//!
//! ```rust
//! use mockd::password;
//!     let upper = true;
//!     let numeric = true;
//!     let special = true;
//!     let num = 27;
//!
//!     let data = password::generate(upper, numeric, special, num); // #9e1Vv5s&Ng8L-#9@=!6+s1+0@R
//! ```

use crate::misc;

/// Pick a random dog breed from the dog breed dictionary.
///
/// # inputs
/// * upper - if true include upper case letters
/// * numeric - if true include numbers
/// * special - if true include symbols
/// * num - number of characters in password
///
/// The generator always uses lower case letters and enforces a minimum
/// password length of five characters.
///
/// # Example
///
/// ```rust
/// let password = mockd::password::generate(true, true, false, 10);
///
/// println!("Password: {}", password);
/// ```
///
pub fn generate(upper: bool, numeric: bool, special: bool, mut num: i8) -> String {
    if num < 5 {
        num = 5;
    }
    letter(upper, numeric, special, num)
}

fn letter(upper: bool, numeric: bool, special: bool, num: i8) -> String {
    let mut pw = String::from("");

    let mut opts = vec![1i8];
    if upper {
        opts.push(2);
    }
    if numeric {
        opts.push(3);
    }
    if special {
        opts.push(4);
    }

    let lower_str = b"abcdefghijklmnopqrstuvwxyz";
    let upper_str = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let numeric_str = b"0123456789";
    let special_str = b"!@#$%&*+-=?";

    match opts[misc::random_data_index(opts.as_slice())] {
        1 => pw.push(misc::random_char_from_string(lower_str)),
        2 => pw.push(misc::random_char_from_string(upper_str)),
        3 => pw.push(misc::random_char_from_string(numeric_str)),
        4 => pw.push(misc::random_char_from_string(special_str)),
        _ => pw.push(misc::random_char_from_string(lower_str)),
    }

    match num {
        0 => pw,
        _ => format!("{}{}", pw, letter(upper, numeric, special, num - 1)),
    }
}

#[cfg(test)]
mod tests {
    use crate::password;
    use crate::testify::exec_mes;

    #[test]
    fn password_generate() {
        exec_mes("password::generate", || {
            password::generate(true, true, true, 26)
        });
    }
}
