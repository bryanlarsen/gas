use super::Mutation;
use crate::chromosone::Gene;

#[mockall_double::double]
use crate::rando::Rando;

/**
*  This mutator randomly changes `n` locuses in the chromosone.
**/
pub struct Null<const N: usize, const NSYMS: usize> {}

impl<const N: usize, const NSYMS: usize> Null<N, NSYMS> {
    pub const fn new() -> Null<N, NSYMS> {
        Null {}
    }
}

impl<const N: usize, const NSYMS: usize> Mutation<N, NSYMS> for Null<N, NSYMS> {
    fn run(&self, chromosone: &[Gene; N], _rng: &mut Rando) -> [Gene; N] {
        chromosone.clone()
    }
}
