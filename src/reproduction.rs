pub mod mutate;

use crate::candidate::*;
use crate::fitness::*;
use crate::rando::*;

pub trait Mutation {
    fn run(
        &self,
        candidate: &Candidate,
        score_config: &[Box<dyn FitnessFunction>],
        rng: &mut dyn Rando,
    ) -> Candidate;
}
