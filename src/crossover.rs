pub mod mix;
pub mod null;
pub mod splice;

use crate::chromosone::Chromosone;

#[mockall_double::double]
use crate::rando::Rando;

/**
*  An operator that given two chromosones, produces a third.   Aka breeding.
**/
pub trait Crossover {
    fn run(&self, left: &Chromosone, right: &Chromosone, rng: &mut Rando) -> Chromosone;
}

pub struct CrossoverIter<'a> {
    i: usize,
    config: &'a CrossoverConfig,
}

impl<'a> Iterator for CrossoverIter<'a> {
    type Item = &'a Box<dyn Crossover + Sync + Send>;

    fn next(&mut self) -> Option<&'a Box<dyn Crossover + Sync + Send>> {
        self.i += 1;
        if self.i >= self.config.indices.len() {
            self.i = 0;
        }
        Some(&self.config.crossovers_with_weights[self.config.indices[self.i]].1)
    }
}

pub struct CrossoverConfig {
    pub crossovers_with_weights: Vec<(usize, Box<dyn Crossover + Sync + Send>)>,
    pub indices: Vec<usize>,
}

impl CrossoverConfig {
    pub fn new(
        crossovers_with_weights: Vec<(usize, Box<dyn Crossover + Sync + Send>)>,
    ) -> CrossoverConfig {
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

    pub fn iter(&self) -> CrossoverIter {
        CrossoverIter {
            i: self.indices.len(),
            config: self,
        }
    }
}
