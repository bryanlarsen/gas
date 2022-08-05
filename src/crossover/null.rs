use crate::candidate::Candidate;
use crate::config::Configuration;
use crate::crossover::Crossover;

#[mockall_double::double]
use crate::rando::Rando;

pub struct Null {}

impl Null {
    pub fn new() -> Self {
        Self {}
    }
}

impl Crossover for Null {
    fn run(
        &self,
        left: &Candidate,
        _right: &Candidate,
        _config: &Configuration,
        _rng: &mut Rando,
    ) -> Candidate {
        left.clone()
    }
}
