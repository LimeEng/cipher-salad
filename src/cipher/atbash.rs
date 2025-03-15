use crate::{Alphabet, ForeignGraphemesPolicy};
use unicode_segmentation::UnicodeSegmentation;

pub struct Atbash {
    foreign_policy: ForeignGraphemesPolicy,
    alphabet: Alphabet,
}

impl Atbash {
    #[must_use]
    pub fn new(alphabet: Alphabet, foreign_policy: ForeignGraphemesPolicy) -> Self {
        Self {
            foreign_policy,
            alphabet,
        }
    }

    fn cipher(&self, contents: &str) -> String {
        contents
            .graphemes(true)
            .map(|grapheme| (grapheme, self.alphabet.index_of(grapheme)))
            .filter_map(|(grapheme, index)| match index {
                Some(grapheme_index) => {
                    let ciphered_index = -(grapheme_index as isize + 1);
                    let ciphered_index = ciphered_index.rem_euclid(self.alphabet.length() as isize);
                    self.alphabet.grapheme_at(ciphered_index as usize).cloned()
                }
                None => match self.foreign_policy {
                    ForeignGraphemesPolicy::Include => Some(grapheme.to_string()),
                    ForeignGraphemesPolicy::Exclude => Some(String::new()),
                },
            })
            .collect()
    }

    #[must_use]
    pub fn encrypt(&self, contents: &str) -> String {
        self.cipher(contents)
    }

    #[must_use]
    pub fn decrypt(&self, ciphertext: &str) -> String {
        self.cipher(ciphertext)
    }
}
