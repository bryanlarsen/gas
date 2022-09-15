pub mod full;
pub mod sample;

pub use full::Full;
pub use sample::Sample;

use crate::candidate::Candidate;

#[mockall_double::double]
use crate::rando::Rando;

#[derive(Debug, PartialEq)]
pub enum LeftRight {
    Left,
    Right,
}

pub trait Game<const N: usize, const NSYMS: usize> {
    /// not all games require an RNG, but most do, and putting the RNG into the constructor means we'd need to use mutable Game's.
    /// similarly, not all games use [Game.score_weights], but some do and all probably should.
    fn run(
        &self,
        left: &Candidate<N, NSYMS>,
        right: &Candidate<N, NSYMS>,
        rng: &mut Rando,
        score_weights: &Vec<f64>,
    ) -> LeftRight;
}
