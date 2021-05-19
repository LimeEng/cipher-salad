pub mod vigenere;

pub use vigenere::*;

enum CipherOperation {
    Encrypt,
    Decrypt,
}
