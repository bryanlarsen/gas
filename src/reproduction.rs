pub mod mix;
pub mod mutate;
pub mod rotate;
pub mod splice;

use crate::candidate::Candidate;
use crate::config::Configuration;

#[mockall_double::double]
use crate::rando::Rando;

pub trait Mutation {
    fn run(&self, candidate: &Candidate, config: &Configuration, rng: &mut Rando) -> Candidate;
}

pub trait Crossover {
    fn run(
        &self,
        left: &Candidate,
        right: &Candidate,
        config: &Configuration,
        rng: &mut Rando,
    ) -> Candidate;
}
