use super::Mutation;
use crate::chromosone::Chromosone;

#[mockall_double::double]
use crate::rando::Rando;

/**
*  This mutator randomly changes `n` locuses in the chromosone.
**/
pub struct Null {}

impl Null {
    pub const fn new() -> Null {
        Null {}
    }
}

impl Mutation for Null {
    fn run(&self, chromosone: &Chromosone, _rng: &mut Rando) -> Chromosone {
        chromosone.clone()
    }
}
