pub mod alphabet;
pub mod cipher;

pub use crate::alphabet::Alphabet;

#[derive(Clone, Copy, Debug)]
pub enum ForeignGraphemesPolicy {
    Include,
    Exclude,
}
