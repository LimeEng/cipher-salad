pub mod alphabet;
pub mod cipher;

pub use crate::alphabet::Alphabet;

pub enum ForeignGraphemesPolicy {
    Include,
    Exclude,
}
