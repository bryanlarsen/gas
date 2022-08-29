use crate::config::default::MUTATION_CONFIG;

pub mod mutate;
pub mod rotate;

use crate::candidate::Candidate;

#[mockall_double::double]
use crate::rando::Rando;

pub enum Mutation {
    #[cfg_attr(test, allow(dead_code))]
    Null,
    Mutate(mutate::Mutate),
    #[cfg_attr(test, allow(dead_code))]
    Rotate(rotate::Rotate),
}

impl Mutation {
    pub fn run(&self, candidate: &Candidate, rng: &mut Rando) -> Candidate {
        match self {
            Mutation::Null => candidate.clone(),
            Mutation::Mutate(m) => m.run(candidate, rng),
            Mutation::Rotate(m) => m.run(candidate, rng),
        }
    }
}
pub struct MutationIter {
    pub i: usize,
    pub indexes: Vec<usize>,
}

impl Iterator for MutationIter {
    type Item = &'static Mutation;

    fn next(&mut self) -> Option<&'static Mutation> {
        self.i += 1;
        if self.i >= self.indexes.len() {
            self.i = 0;
        }
        Some(&MUTATION_CONFIG.mutations_with_weights[self.indexes[self.i]].1)
    }
}

pub struct MutationConfig {
    mutations_with_weights: &'static [(usize, Mutation)],
}

impl MutationConfig {
    pub const fn new(mutations_with_weights: &'static [(usize, Mutation)]) -> MutationConfig {
        MutationConfig {
            mutations_with_weights,
        }
    }

    pub fn iter(&self) -> MutationIter {
        let weights = self
            .mutations_with_weights
            .iter()
            .map(|c| c.0)
            .collect::<Vec<usize>>();
        // FIXME.  "indexes" should be const or lazy_static or something rather than being recalculated every time here.   const is best -- multidimensional_bresenhams could theoretically be const, but Vec can't be const and it's hard to rejigger to use something other than Vec.
        let indexes = crate::helpers::multidimensional_bresenhams(&weights);
        MutationIter {
            i: indexes.len(),
            indexes,
        }
    }
}
