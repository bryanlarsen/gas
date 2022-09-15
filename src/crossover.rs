pub mod mix;
pub mod null;
pub mod splice;

pub use mix::Mix;
pub use null::Null;
pub use splice::Splice;

use crate::chromosone::Gene;

#[mockall_double::double]
use crate::rando::Rando;

/**
*  An operator that given two chromosones, produces a third.   Aka breeding.
**/
pub trait Crossover<const N: usize, const NSYMS: usize> {
    fn run(&self, left: &[Gene; N], right: &[Gene; N], rng: &mut Rando) -> [Gene; N];
}

pub struct CrossoverIter<'a, const N: usize, const NSYMS: usize> {
    i: usize,
    config: &'a CrossoverConfig<N, NSYMS>,
}

impl<'a, const N: usize, const NSYMS: usize> Iterator for CrossoverIter<'a, N, NSYMS> {
    type Item = &'a Box<dyn Crossover<N, NSYMS> + Sync + Send>;

    fn next(&mut self) -> Option<&'a Box<dyn Crossover<N, NSYMS> + Sync + Send>> {
        self.i += 1;
        if self.i >= self.config.indices.len() {
            self.i = 0;
        }
        Some(&self.config.crossovers_with_weights[self.config.indices[self.i]].1)
    }
}

pub struct CrossoverConfig<const N: usize, const NSYMS: usize> {
    pub crossovers_with_weights: Vec<(usize, Box<dyn Crossover<N, NSYMS> + Sync + Send>)>,
    pub indices: Vec<usize>,
}

impl<const N: usize, const NSYMS: usize> CrossoverConfig<N, NSYMS> {
    pub fn new(
        crossovers_with_weights: Vec<(usize, Box<dyn Crossover<N, NSYMS> + Sync + Send>)>,
    ) -> CrossoverConfig<N, NSYMS> {
        let weights = crossovers_with_weights
            .iter()
            .map(|c| c.0)
            .collect::<Vec<usize>>();
        let indices = crate::helpers::multidimensional_bresenhams(&weights);
        CrossoverConfig {
            indices,
            crossovers_with_weights,
        }
    }

    pub fn iter(&self) -> CrossoverIter<N, NSYMS> {
        CrossoverIter {
            i: self.indices.len(),
            config: self,
        }
    }
}
