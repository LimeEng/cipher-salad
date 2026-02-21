#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]

mod alphabet;
pub mod cipher;

pub use alphabet::Alphabet;

#[derive(Clone, Copy, Debug)]
pub enum ForeignGraphemesPolicy {
    Include,
    Exclude,
}
