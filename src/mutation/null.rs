use crate::candidate::Candidate;
use crate::config::Configuration;
use crate::mutation::Mutation;

#[mockall_double::double]
use crate::rando::Rando;

pub struct Null {}

impl Null {
    pub fn new() -> Null {
        Null {}
    }
}

impl Mutation for Null {
    fn run(&self, candidate: &Candidate, _config: &Configuration, _rng: &mut Rando) -> Candidate {
        candidate.clone()
    }
}
