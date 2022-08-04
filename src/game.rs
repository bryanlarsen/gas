pub mod full;
pub mod sample;

use crate::candidate::Candidate;

#[mockall_double::double]
use crate::rando::Rando;

#[derive(Debug, PartialEq)]
pub enum LeftRight {
    Left,
    Right,
}

pub trait Game {
    // not all games require an RNG, but most do, and putting the RNG into the constructor means we'd need to use mutable Game's.
    fn run(&self, left: &Candidate, right: &Candidate, rng: &mut Rando) -> LeftRight;
}
