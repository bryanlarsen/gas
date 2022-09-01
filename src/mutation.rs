pub mod mutate;
pub mod null;
pub mod rotate;

use crate::chromosone::Chromosone;

#[mockall_double::double]
use crate::rando::Rando;

pub trait Mutation {
    fn run(&self, candidate: &Chromosone, rng: &mut Rando) -> Chromosone;
}

pub struct MutationIter<'a> {
    i: usize,
    config: &'a MutationConfig,
}

impl<'a> Iterator for MutationIter<'a> {
    type Item = &'a Box<dyn Mutation + Sync + Send>;

    fn next(&mut self) -> Option<&'a Box<dyn Mutation + Sync + Send>> {
        self.i += 1;
        if self.i >= self.config.indices.len() {
            self.i = 0;
        }
        Some(&self.config.mutations_with_weights[self.config.indices[self.i]].1)
    }
}

pub struct MutationConfig {
    mutations_with_weights: Vec<(usize, Box<dyn Mutation + Sync + Send>)>,
    indices: Vec<usize>,
}

impl MutationConfig {
    pub fn new(
        mutations_with_weights: Vec<(usize, Box<dyn Mutation + Sync + Send>)>,
    ) -> MutationConfig {
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

    pub fn iter(&self) -> MutationIter {
        MutationIter {
            i: self.indices.len(),
            config: self,
        }
    }
}
