pub mod mix;
pub mod null;
pub mod splice;

use crate::candidate::Candidate;
use crate::config::Configuration;

#[mockall_double::double]
use crate::rando::Rando;

pub trait Crossover {
    fn run(
        &self,
        left: &Candidate,
        right: &Candidate,
        config: &Configuration,
        rng: &mut Rando,
    ) -> Candidate;
}
