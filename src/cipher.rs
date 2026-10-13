mod atbash;
mod caesar;
mod vigenere;

pub use atbash::Atbash;
pub use caesar::Caesar;
pub use vigenere::{Vigenere, VigenereError};

#[derive(Clone, Copy, Debug)]
enum CipherOperation {
    Encrypt,
    Decrypt,
}
