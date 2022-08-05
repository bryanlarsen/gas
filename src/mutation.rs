pub mod mutate;
pub mod null;
pub mod rotate;

use crate::candidate::Candidate;
use crate::config::Configuration;

#[mockall_double::double]
use crate::rando::Rando;

pub trait Mutation {
    fn run(&self, candidate: &Candidate, config: &Configuration, rng: &mut Rando) -> Candidate;
}
