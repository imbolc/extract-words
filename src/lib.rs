//! # extract-words
//!
//! Extracts words from a phrase, omitting any punctuation, without allocation
//!
//! ```
//! # use extract_words::extract_words;
//! assert_eq!(
//!     extract_words("Hola, mundo!").collect::<Vec<_>>(),
//!     ["Hola", "mundo"]
//! );
//! ```

#![warn(clippy::all, missing_docs, nonstandard_style, future_incompatible)]

/// Extracts words from a phrase, discarding punctuation
pub fn extract_words(phrase: &str) -> WordIter {
    WordIter::new(phrase)
}

/// An iterator over the words in a phrase created by the [`extract_words`] function
pub struct WordIter<'a> {
    phrase: &'a str,
    char_indices: std::str::CharIndices<'a>,
    word_start: Option<usize>,
}

impl<'a> WordIter<'a> {
    fn new(phrase: &'a str) -> Self {
        WordIter {
            phrase,
            char_indices: phrase.char_indices(),
            word_start: None,
        }
    }

    #[inline]
    fn in_word(&self) -> bool {
        self.word_start.is_some()
    }
}

impl<'a> Iterator for WordIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((i, c)) = self.char_indices.next() {
            if c.is_alphanumeric() {
                if !self.in_word() {
                    self.word_start = Some(i);
                }
                continue;
            }

            if let Some(start) = self.word_start.take() {
                return Some(&self.phrase[start..i]);
            }
        }

        if let Some(start) = self.word_start.take() {
            if start < self.phrase.len() {
                return Some(&self.phrase[start..]);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::extract_words;

    fn extract_vec(phrase: &str) -> Vec<&str> {
        extract_words(phrase).collect()
    }

    #[test]
    fn test_empty_string() {
        assert!(extract_vec("").is_empty());
    }

    #[test]
    fn test_punctuation_only() {
        assert!(extract_vec(".,!?-").is_empty());
    }

    #[test]
    fn test_mixed_input() {
        assert_eq!(
            extract_vec("Hola,mundo! ¿Cómo estás?"),
            ["Hola", "mundo", "Cómo", "estás"]
        );
    }

    #[test]
    fn test_multiple_delimiters() {
        assert_eq!(extract_vec("Hola, mundo!¿ .. !¿á"), ["Hola", "mundo", "á"]);
    }
}
