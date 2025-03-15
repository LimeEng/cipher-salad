use crate::{
    Alphabet, ForeignGraphemesPolicy,
    cipher::CipherOperation::{self, Decrypt, Encrypt},
};
use unicode_segmentation::UnicodeSegmentation;

pub struct Caesar {
    foreign_policy: ForeignGraphemesPolicy,
    alphabet: Alphabet,
}

impl Caesar {
    #[must_use]
    pub fn new(alphabet: Alphabet, foreign_policy: ForeignGraphemesPolicy) -> Self {
        Self {
            foreign_policy,
            alphabet,
        }
    }

    fn cipher(&self, contents: &str, key: usize, operation: CipherOperation) -> String {
        let key = key % self.alphabet.length();

        contents
            .graphemes(true)
            .map(|grapheme| (grapheme, self.alphabet.index_of(grapheme)))
            .filter_map(|(grapheme, index)| match index {
                Some(grapheme_index) => {
                    let ciphered_index = match operation {
                        Encrypt => (grapheme_index + key).rem_euclid(self.alphabet.length()),
                        Decrypt => (grapheme_index as isize - key as isize)
                            .rem_euclid(self.alphabet.length() as isize)
                            as usize,
                    };
                    self.alphabet.grapheme_at(ciphered_index).cloned()
                }
                None => match self.foreign_policy {
                    ForeignGraphemesPolicy::Include => Some(grapheme.to_string()),
                    ForeignGraphemesPolicy::Exclude => Some(String::new()),
                },
            })
            .collect()
    }

    #[must_use]
    pub fn encrypt(&self, contents: &str, key: usize) -> String {
        self.cipher(contents, key, CipherOperation::Encrypt)
    }

    #[must_use]
    pub fn decrypt(&self, ciphertext: &str, key: usize) -> String {
        self.cipher(ciphertext, key, CipherOperation::Decrypt)
    }
}
