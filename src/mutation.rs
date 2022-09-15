pub mod mutate;
pub mod null;
pub mod rotate;

pub use mutate::Mutate;
pub use null::Null;
pub use rotate::Rotate;

use crate::chromosone::Gene;

#[mockall_double::double]
use crate::rando::Rando;

pub trait Mutation<const N: usize, const NSYMS: usize> {
    fn run(&self, candidate: &[Gene; N], rng: &mut Rando) -> [Gene; N];
}

pub struct MutationIter<'a, const N: usize, const NSYMS: usize> {
    i: usize,
    config: &'a MutationConfig<N, NSYMS>,
}

impl<'a, const N: usize, const NSYMS: usize> Iterator for MutationIter<'a, N, NSYMS> {
    type Item = &'a Box<dyn Mutation<N, NSYMS> + Sync + Send>;

    fn next(&mut self) -> Option<&'a Box<dyn Mutation<N, NSYMS> + Sync + Send>> {
        self.i += 1;
        if self.i >= self.config.indices.len() {
            self.i = 0;
        }
        Some(&self.config.mutations_with_weights[self.config.indices[self.i]].1)
    }
}

pub struct MutationConfig<const N: usize, const NSYMS: usize> {
    mutations_with_weights: Vec<(usize, Box<dyn Mutation<N, NSYMS> + Sync + Send>)>,
    indices: Vec<usize>,
}

impl<const N: usize, const NSYMS: usize> MutationConfig<N, NSYMS> {
    pub fn new(
        mutations_with_weights: Vec<(usize, Box<dyn Mutation<N, NSYMS> + Sync + Send>)>,
    ) -> MutationConfig<N, NSYMS> {
        let weights = mutations_with_weights
            .iter()
            .map(|c| c.0)
            .collect::<Vec<usize>>();
        let indices = crate::helpers::multidimensional_bresenhams(&weights);
        MutationConfig {
            indices,
            mutations_with_weights,
        }
    }

    pub fn iter(&self) -> MutationIter<N, NSYMS> {
        MutationIter {
            i: self.indices.len(),
            config: self,
        }
    }
}
