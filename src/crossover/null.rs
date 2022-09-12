use crate::chromosone::Gene;
use crate::crossover::Crossover;

#[mockall_double::double]
use crate::rando::Rando;

pub struct Null<const N: usize, const NSYMS: usize> {}

impl<const N: usize, const NSYMS: usize> Null<N, NSYMS> {
    pub const fn new() -> Self {
        Self {}
    }
}

impl<const N: usize, const NSYMS: usize> Crossover<N, NSYMS> for Null<N, NSYMS> {
    fn run(&self, left: &[Gene; N], _right: &[Gene; N], _rng: &mut Rando) -> [Gene; N] {
        left.clone()
    }
}
