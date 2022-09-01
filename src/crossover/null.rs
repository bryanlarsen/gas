use crate::chromosone::Chromosone;
use crate::crossover::Crossover;

#[mockall_double::double]
use crate::rando::Rando;

pub struct Null {}

impl Null {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Crossover for Null {
    fn run(&self, left: &Chromosone, _right: &Chromosone, _rng: &mut Rando) -> Chromosone {
        left.clone()
    }
}
