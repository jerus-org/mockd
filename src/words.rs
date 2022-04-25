//!
//! Provides 6 functions to return mock word data.
//!
//! # Examples
//!
//! ```rust
//! use mockd::words;
//!
//!     let word_count = 11;
//!     let count = 4;
//!     let sentence_count = 5;
//!     let separator = "/n".to_string();
//!
//!     let data = words::word(); // word: saepe
//!     let data = words::sentence(word_count); // sentence: Nemo vitae rerum consequuntur vero animi incidunt esse doloribus eos.
//!     let data = words::paragraph(count, sentence_count, word_count, separator); // paragraph: Minima aut numquam nihil rerum commodi pariatur dolores...
//!     let data = words::question(); // question: Placeat voluptatem at ut eveniet suscipit similique dicta quis?
//!     let data = words::quote(); // quote: "Dignissimos dolorem quam tempore excepturi facere dicta." - Willy Kihn
//!
//!     let opts = words::ParagraphOpts::new(5,4,11,"\n");
//!     let data = words::paragraph_generator(opts, &words::sentence); // paragraph_generator: Quisquam aut consequuntur nobis voluptas porro...
//! ```
//!

use crate::data::lorem;
use crate::hipster;
use crate::misc;
use crate::name;

/// Struct to describe the options that are required to generate a paragraph.
///
/// * count - the number of paragraphs
/// * sentence_count - the number of sentences in each paragraph
/// * word_count - the number of words in each sentence
/// * separator - the separator between the paragraphs
///
#[derive(Debug)]
pub struct ParagraphOpts {
    count: i64,
    sentence_count: i64,
    word_count: i64,
    separator: String,
}

impl ParagraphOpts {
    /// Initialise a new paragraph opts struct.
    ///
    /// # inputs
    /// * count - the number of paragraphs
    /// * sentence_count - the number of sentences in each paragraph
    /// * word_count - the number of words in each sentence
    /// * separator - the separator between the paragraphs
    ///
    /// # Example
    ///
    /// ```rust
    /// use mockd::words::ParagraphOpts;
    ///
    /// let options = ParagraphOpts::new(5,4,11,"\n");
    ///
    /// println!("Options: {:#?}", options);
    /// ```
    ///
    pub fn new(count: i64, sentence_count: i64, word_count: i64, sep: &str) -> ParagraphOpts {
        ParagraphOpts {
            count,
            sentence_count,
            word_count,
            separator: sep.to_string(),
        }
    }
}

/// Pick a random word from the word dictionary.
///
/// # Example
///
/// ```rust
/// let word = mockd::words::word();
///
/// println!("Word: {}", word);
/// ```
///
pub fn word() -> String {
    misc::random_data(lorem::WORD).to_string()
}

/// Generate a random sentence containing the specified number of words.
///
/// # Example
///
/// ```rust
/// let sentence = mockd::words::sentence(5);
///
/// println!("Sentence: {}", sentence);
/// ```
///
pub fn sentence(word_count: i64) -> String {
    if word_count <= 0 {
        return "".to_string();
    }

    let mut sentence_vec = Vec::<String>::new();
    for i in 0..word_count {
        if i == 0 {
            sentence_vec.push(title(word()))
        } else if i == word_count - 1 {
            let word_with_dot = format!("{}.", word());
            sentence_vec.push(word_with_dot)
        } else {
            sentence_vec.push(word())
        }
    }

    sentence_vec.join(" ")
}

/// Generate one or more random paragraphs in a single string.
///
/// # inputs
/// * count - the number of paragraphs
/// * sentence_count - the number of sentences in each paragraph
/// * word_count - the number of words in each sentence
/// * separator - the separator between the paragraphs
///
/// # Example
///
/// ```rust
/// let paragraphs = mockd::words::paragraph(5,4,11,"\n".to_string());
///
/// println!("Paragraphs: {}", paragraphs);
/// ```
///
pub fn paragraph(count: i64, sentence_count: i64, word_count: i64, separator: String) -> String {
    let opts = ParagraphOpts::new(count, sentence_count, word_count, &separator);

    paragraph_generator(opts, &sentence)
}

/// Generate one or more paragraphs.
///
/// # Inputs
/// * opts - options struct containing the configuration for the paragraphs.
/// * sentence_generator - function to use to generate the sentences.
///
/// # Example
///
/// ```rust
/// use mockd::words::ParagraphOpts;
///
/// let options = ParagraphOpts::new(5,4,11,"\n");
/// let paragraph = mockd::words::paragraph_generator(
///                         options,
///                         &mockd::words::sentence);
///
/// println!("Paragraph: {}", paragraph);
/// ```
///
pub fn paragraph_generator(
    opts: ParagraphOpts,
    sentence_generator: &dyn Fn(i64) -> String,
) -> String {
    let mut paragraph_vec = Vec::<String>::new();
    for _i in 0..opts.count {
        let mut sentence_vec = Vec::<String>::new();
        for _i in 0..opts.sentence_count {
            sentence_vec.push(sentence_generator(opts.word_count));
        }
        paragraph_vec.push(sentence_vec.join(" "))
    }
    paragraph_vec.join(&opts.separator[..])
}

/// Generate a random question.
///
/// # Example
///
/// ```rust
/// let question = mockd::words::question();
///
/// println!("Question: {}", question);
/// ```
///
pub fn question() -> String {
    hipster::sentence(misc::random(3, 10)).replace('.', "?")
}

/// Generate a random quote.
///
/// # Example
///
/// ```rust
/// let quote = mockd::words::quote();
///
/// println!("Quote: {}", quote);
/// ```
///
pub fn quote() -> String {
    format!(
        "\"{}\" - {} {}",
        hipster::sentence(misc::random(3, 10)),
        name::first(),
        name::last()
    )
}

fn title(s: String) -> String {
    let mut v: Vec<char> = s.chars().collect();
    v[0] = v[0].to_uppercase().next().unwrap();
    v.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::testify::exec_mes;
    use crate::words;

    #[test]
    fn word() {
        exec_mes("words::word", words::word);
    }

    #[test]
    fn sentence() {
        exec_mes("words::sentence", || words::sentence(10));
    }

    #[test]
    fn paragraph() {
        exec_mes("words::paragraph", || {
            words::paragraph(5, 4, 11, "\n".to_string())
        });
    }

    #[test]
    fn question() {
        exec_mes("words::question", words::question);
    }

    #[test]
    fn quote() {
        exec_mes("words::quote", words::quote);
    }

    #[test]
    fn paragraph_generator() {
        exec_mes("words::paragraph_generator", || {
            let opts = words::ParagraphOpts {
                count: 5,
                sentence_count: 4,
                word_count: 11,
                separator: "\n".to_string(),
            };
            words::paragraph_generator(opts, &words::sentence)
        });
    }
}
