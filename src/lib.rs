//! # extract-words
//!
//! Extracts words from text without allocation
//!
//! ## Examples
//!
//! Iteration through words, discarding punctuation
//! ```
//! # use extract_words::extract_words;
//! let mut words = extract_words("¿Cómo estás?");
//! assert_eq!(words.next().unwrap(), "Cómo");
//! assert_eq!(words.next().unwrap(), "estás");
//! assert!(words.next().is_none());
//! ```
//!
//! Iteration through all entries
//! ```
//! # use extract_words::{Entries, Entry};
//! let mut entries = Entries::new("Bien :)");
//! assert_eq!(entries.next().unwrap(), Entry::Word("Bien"));
//! assert_eq!(entries.next().unwrap(), Entry::Other(" :)"));
//! assert!(entries.next().is_none());
//! ```

#![warn(clippy::all, missing_docs, nonstandard_style, future_incompatible)]

/// Extracts words from the text discarding punctuation
pub fn extract_words(text: &str) -> impl Iterator<Item = &str> {
    Entries::new(text).filter_map(|e| match e {
        Entry::Word(s) => Some(s),
        Entry::Other(_) => None,
    })
}

/// An iterator over text entries
pub struct Entries<'a> {
    text: &'a str,
    char_indices: std::str::CharIndices<'a>,
    cur_entry: CurEntry,
}

/// Text entry
#[derive(Debug, PartialEq)]
pub enum Entry<'a> {
    /// Punctuation, spaces, etc
    Other(&'a str),
    /// Word
    Word(&'a str),
}

enum CurEntry {
    None,
    Other(usize),
    Word(usize),
}

impl<'a> Entries<'a> {
    /// Creates an iterator over the text entries
    pub fn new(text: &'a str) -> Self {
        Entries {
            text,
            char_indices: text.char_indices(),
            cur_entry: CurEntry::None,
        }
    }
}

impl<'a> Iterator for Entries<'a> {
    type Item = Entry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        for (i, c) in self.char_indices.by_ref() {
            if c.is_alphanumeric() {
                match self.cur_entry {
                    CurEntry::None => self.cur_entry = CurEntry::Word(i),
                    CurEntry::Other(start) => {
                        self.cur_entry = CurEntry::Word(i);
                        return Some(Entry::Other(&self.text[start..i]));
                    }
                    CurEntry::Word(_) => (),
                }
            } else {
                match self.cur_entry {
                    CurEntry::None => self.cur_entry = CurEntry::Other(i),
                    CurEntry::Other(_) => (),
                    CurEntry::Word(start) => {
                        self.cur_entry = CurEntry::Other(i);
                        return Some(Entry::Word(&self.text[start..i]));
                    }
                }
            }
        }

        match self.cur_entry {
            CurEntry::None => None,
            CurEntry::Other(start) => {
                self.cur_entry = CurEntry::None;
                if start < self.text.len() {
                    Some(Entry::Other(&self.text[start..]))
                } else {
                    None
                }
            }
            CurEntry::Word(start) => {
                self.cur_entry = CurEntry::None;
                if start < self.text.len() {
                    Some(Entry::Word(&self.text[start..]))
                } else {
                    None
                }
            }
        }
    }
}

impl<'a> AsRef<str> for Entry<'a> {
    fn as_ref(&self) -> &str {
        match self {
            Entry::Other(s) => s,
            Entry::Word(s) => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::extract_words;

    fn extract_vec(text: &str) -> Vec<&str> {
        extract_words(text).collect()
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
