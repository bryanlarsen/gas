pub mod mutate;

use crate::candidate::Candidate;
use crate::config::Configuration;
use crate::rando::Rando;

pub trait Mutation {
    fn run(&self, candidate: &Candidate, config: &Configuration, rng: &mut dyn Rando) -> Candidate;
}
