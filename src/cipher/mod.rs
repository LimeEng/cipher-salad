pub mod atbash;
pub mod caesar;
pub mod vigenere;

pub use atbash::*;
pub use caesar::*;
pub use vigenere::*;

#[derive(Clone, Copy, Debug)]
enum CipherOperation {
    Encrypt,
    Decrypt,
}
