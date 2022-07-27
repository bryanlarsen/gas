#[cfg(not(test))]
use crate::data::*;
#[cfg(test)]
use crate::test_data::*;

use crate::candidate::Candidate;
use crate::config::Configuration;
use crate::rando::*;
use crate::reproduction::Mutation;

#[cfg(test)]
use mockall::*;

pub struct Mutate {
    pub n: usize,
}

impl Mutate {
    pub fn new(n: usize) -> Mutate {
        Mutate { n }
    }
}

impl Mutation for Mutate {
    fn run(&self, candidate: &Candidate, config: &Configuration, rng: &mut dyn Rando) -> Candidate {
        let mut mutated = [0usize; LENGTH];
        mutated.copy_from_slice(&candidate.chromosone);
        for _ in 0..self.n {
            let pos = rng.gen_range(0..LENGTH);
            let mut new;
            loop {
                new = rng.gen_range(0..NSYMS);
                if new != candidate.chromosone[pos] {
                    break;
                }
            }
            mutated[pos] = new;
        }
        Candidate::from_chromosone(mutated, config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutate() {
        let config = configuration();
        let mut r = MockRando::new();
        let m = Mutate::new(1);
        r.expect_gen_range()
            .with(predicate::eq(0..LENGTH))
            .times(1)
            .return_const(1usize);
        r.expect_gen_range()
            .with(predicate::eq(0..NSYMS))
            .times(1)
            .return_const(2usize);
        Candidate::assert_eq(
            &m.run(
                &Candidate::from_chromosone([0, 1, 2, 0, 1], &config),
                &config,
                &mut r,
            ),
            &Candidate::from_chromosone([0, 2, 2, 0, 1], &config),
        );
    }
}
