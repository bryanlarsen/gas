use crate::config::config::CROSSOVER_CONFIG;

pub mod mix;
pub mod splice;

use crate::candidate::Candidate;

#[mockall_double::double]
use crate::rando::Rando;

pub enum Crossover {
    Null,
    #[cfg_attr(test, allow(dead_code))]
    Mix(mix::Mix),
    #[cfg_attr(test, allow(dead_code))]
    Splice(splice::Splice),
}

impl Crossover {
    pub fn run(&self, left: &Candidate, right: &Candidate, rng: &mut Rando) -> Candidate {
        match self {
            Crossover::Null => left.clone(),
            Crossover::Mix(co) => co.run(left, right, rng),
            Crossover::Splice(co) => co.run(left, right, rng),
        }
    }
}

pub struct CrossoverIter {
    pub i: usize,
    pub indexes: Vec<usize>,
}

impl Iterator for CrossoverIter {
    type Item = &'static Crossover;

    fn next(&mut self) -> Option<&'static Crossover> {
        self.i += 1;
        if self.i >= self.indexes.len() {
            self.i = 0;
        }
        Some(&CROSSOVER_CONFIG.crossovers_with_weights[self.indexes[self.i]].1)
    }
}

pub struct CrossoverConfig {
    crossovers_with_weights: &'static [(usize, Crossover)],
}

impl CrossoverConfig {
    pub const fn new(crossovers_with_weights: &'static [(usize, Crossover)]) -> CrossoverConfig {
        CrossoverConfig {
            crossovers_with_weights,
        }
    }

    pub fn iter(&self) -> CrossoverIter {
        let weights = self
            .crossovers_with_weights
            .iter()
            .map(|c| c.0)
            .collect::<Vec<usize>>();
        // FIXME.  "indexes" should be const or lazy_static or something rather than being recalculated every time here.   const is best -- multidimensional_bresenhams could theoretically be const, but Vec can't be const and it's hard to rejigger to use something other than Vec.
        let indexes = crate::helpers::multidimensional_bresenhams(&weights);
        CrossoverIter {
            i: indexes.len(),
            indexes,
        }
    }
}
