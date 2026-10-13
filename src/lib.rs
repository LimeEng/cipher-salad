//! Implementations of classic ciphers.

mod alphabet;
pub mod cipher;

pub use alphabet::Alphabet;

#[derive(Clone, Copy, Debug)]
pub enum ForeignGraphemesPolicy {
    Include,
    Exclude,
}
