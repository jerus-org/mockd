//!
//! Provides 1 function to generate a string based on input string.
//!
//! # Tags
//!
//! The input string can contain the following tags:
//! * contact.email
//! * hacker.abbreviation
//! * hacker.adjective
//! * hacker.noun
//! * hacker.verb
//! * hacker.ingverb
//! * person.first
//! * person.last
//!
//! # Letters and Numbers
//!
//! Hashtags (#) are replaced with numbers and question marks are
//! replaced with letters.
//!
//! # Examples
//!
//! ```rust
//! use mockd::generator;
//!
//!     let data = generator::generate("{person.first} {person.last} {contact.email} #?#?#?".to_string()); // data: Watson Connelly baileeprosacco@smitham.biz 6d0e0a
//!     // More details about this later
//! ```

use crate::contact;
use crate::hacker;
use crate::person::data as person_data;

use crate::misc;

///
/// Provides 1 function to generate a string based on input string.
///
/// # Tags
///
/// The input string can contain the following tags:
/// * contact.email
/// * hacker.abbreviation
/// * hacker.adjective
/// * hacker.noun
/// * hacker.verb
/// * hacker.ingverb
/// * person.first
/// * person.last
///
/// # Letters and Numbers
///
/// Hashtags (#) are replaced with numbers and question marks are
/// replaced with letters.
///
/// # Examples
///
/// ```rust
/// use mockd::generator;
///
///     let data = generator::generate("{person.first} {person.last} {contact.email} #?#?#?".to_string()); // data: Watson Connelly baileeprosacco@smitham.biz 6d0e0a
///     // More details about this later
/// ```
///
pub fn generate(data: String) -> String {
    let mut d_validate_left = data.matches('{').count();
    let mut d_validate_right = data.matches('}').count();

    let mut m_data = data; // .clone();

    while d_validate_left > 0 && d_validate_right > 0 && d_validate_left == d_validate_right {
        let place_of_left = m_data.find('{').unwrap();
        let place_of_right = m_data.find('}').unwrap();

        let tag = &m_data[place_of_left + 1..place_of_right];
        let text = resolve_tag(tag);
        let tag_formatted = format!("{}{}{}", "{", tag, "}");
        m_data = m_data.replace(&tag_formatted[..], &text[..]);

        d_validate_left = m_data.matches('{').count();
        d_validate_right = m_data.matches('}').count();
    }

    m_data = misc::replace_with_numbers(m_data);
    m_data = misc::replace_with_letter(m_data);

    m_data
}

fn resolve_tag(tag: &str) -> String {
    match tag {
        "contact.email" => contact::email(),
        "hacker.abbreviation" => hacker::abbreviation(),
        "hacker.adjective" => hacker::adjective(),
        "hacker.noun" => hacker::noun(),
        "hacker.verb" => hacker::verb(),
        "hacker.ingverb" => hacker::ingverb(),
        "person.first" => misc::random_data(person_data::FIRST).to_string(),
        "person.last" => misc::random_data(person_data::LAST).to_string(),

        _ => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::generator;
    use crate::testify::exec_mes;

    #[test]
    fn generate() {
        exec_mes("beer::name", || {
            generator::generate("{person.first} {person.last} {contact.email} #?#?#?".to_string())
        });
    }
}
