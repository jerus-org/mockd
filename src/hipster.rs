//!
//! Provides 3 functions to return mock hipster data.
//!
//! # Examples
//!
//! ```rust
//! use mockd::hipster;
//!
//!     let data = hipster::word(); // word: fingerstache
//!     let data = hipster::sentence(12); // sentence: Itaque aliquid id ex repudiandae adipisci quibusdam excepturi deleniti qui alias animi.
//!     let data = hipster::paragraph(3, 4, 40, " ".to_string()); // paragraph: Voluptas minima delectus voluptatibus earum rerum accusamus consequatur sunt....
//! ```

use crate::data::hipster;
use crate::misc;
use crate::words;

/// Generate a random hipster word from the word dictionary.
///
/// # Example
///
/// ```rust
/// let word = mockd::hipster::word();
///
/// println!("Hipster word: {}", word);
/// ```
///
pub fn word() -> String {
    misc::random_data(hipster::WORD).to_string()
}

/// Generate a random sentence containing the given number of words.
///
/// # Example
///
/// ```rust
/// let sentence = mockd::hipster::sentence();
///
/// println!("Hipster sentence: {}", sentence);
/// ```
///
pub fn sentence(word_count: i64) -> String {
    words::sentence(word_count)
}

/// Generate a random paragraph containing a number of sentences of words.
///
/// # Example
///
/// ```rust
/// let paragraph = mockd::hipster::paragraph();
///
/// println!("Hipster word: {}", paragrpah);
/// ```
///
pub fn paragraph(count: i64, sentence_count: i64, word_count: i64, separator: String) -> String {
    words::paragraph(count, sentence_count, word_count, separator)
}

#[cfg(test)]
mod tests {
    use crate::hipster;
    use crate::testify::exec_mes;

    #[test]
    fn word() {
        exec_mes("hipster::word", hipster::word);
    }

    #[test]
    fn sentence() {
        exec_mes("hipster::sentence", || hipster::sentence(12));
    }

    #[test]
    fn paragraph() {
        let data1 = hipster::paragraph(3, 4, 40, " ".to_string());
        let data2 = hipster::paragraph(3, 4, 40, " ".to_string());
        assert_ne!(data1, data2);
    }
}
