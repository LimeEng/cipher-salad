use crate::{
    Alphabet, ForeignGraphemesPolicy,
    cipher::CipherOperation::{self, Decrypt, Encrypt},
};
use unicode_segmentation::UnicodeSegmentation;

pub struct Vigenere {
    foreign_policy: ForeignGraphemesPolicy,
    alphabet: Alphabet,
}

#[derive(Debug)]
pub enum VigenereError {
    InvalidKey,
}

impl Vigenere {
    #[must_use]
    pub fn new(alphabet: Alphabet, foreign_policy: ForeignGraphemesPolicy) -> Self {
        Self {
            foreign_policy,
            alphabet,
        }
    }

    fn cipher(
        &self,
        contents: &str,
        key: &str,
        operation: CipherOperation,
    ) -> Result<String, VigenereError> {
        let valid_key = key
            .graphemes(true)
            .all(|grapheme| self.alphabet.contains(grapheme));
        if !valid_key {
            return Err(VigenereError::InvalidKey);
        }

        let approx_length = contents.len();
        let mut result = String::with_capacity(approx_length);

        let mut contents = contents.graphemes(true);
        let mut key = key.graphemes(true).cycle();

        loop {
            let next = contents.next();
            let plain_grapheme = match next {
                Some(p) => p,
                None => break,
            };

            if !self.alphabet.contains(plain_grapheme) {
                match self.foreign_policy {
                    ForeignGraphemesPolicy::Include => result.push_str(plain_grapheme),
                    ForeignGraphemesPolicy::Exclude => (),
                }
                continue;
            }

            let key_grapheme = key.next().unwrap();

            let plain_index = self.alphabet.index_of(plain_grapheme);
            let key_index = self.alphabet.index_of(key_grapheme);

            let ciphered = plain_index
                .zip(key_index)
                .and_then(|(plain_index, key_index)| {
                    let ciphered_index = match operation {
                        Encrypt => (plain_index + key_index).rem_euclid(self.alphabet.length()),
                        Decrypt => {
                            // Vecs in Rust are limited to max isize::MAX elements
                            // https://doc.rust-lang.org/nomicon/vec-alloc.html
                            // As such we must limit all allocations to isize::MAX elements
                            // This is why it is possible to cast to an isize "losslessly"
                            let plain_index = plain_index as isize;
                            let key_index = key_index as isize;
                            // rem_euclid has slightly different semantics than %, which in this case is what is needed
                            (plain_index - key_index).rem_euclid(self.alphabet.length() as isize)
                                as usize
                        }
                    };
                    self.alphabet.grapheme_at(ciphered_index)
                })
                .cloned()
                .unwrap_or_else(String::new);
            result.push_str(&ciphered);
        }
        Ok(result)
    }

    pub fn encrypt(&self, contents: &str, key: &str) -> Result<String, VigenereError> {
        self.cipher(contents, key, CipherOperation::Encrypt)
    }

    pub fn decrypt(&self, ciphertext: &str, key: &str) -> Result<String, VigenereError> {
        self.cipher(ciphertext, key, CipherOperation::Decrypt)
    }
}
