#[cfg(not(test))]
use crate::data::*;
#[cfg(test)]
use crate::test_data::*;

use crate::candidate::*;
use crate::fitness::FitnessFunction;
use crate::rando::*;
use crate::reproduction::*;

#[cfg(test)]
use mockall::*;

pub struct Mutate1 {}

impl Mutate1 {
    pub fn new() -> Mutate1 {
        Mutate1 {}
    }
}

impl Mutation for Mutate1 {
    //pub fn run(chromosone: &[usize; LENGTH], rng: &mut dyn Rando) -> [usize; LENGTH] {
    fn run(
        &self,
        candidate: &Candidate,
        score_config: &[Box<dyn FitnessFunction>],
        rng: &mut dyn Rando,
    ) -> Candidate {
        let pos = rng.gen_range(0..LENGTH);
        let mut new = rng.gen_range(0..NSYMS);
        while new == candidate.chromosone[pos] {
            new = rng.gen_range(0..NSYMS);
        }
        let mut mutated = [0usize; LENGTH];
        mutated.copy_from_slice(&candidate.chromosone);
        mutated[pos] = new;
        Candidate::from_chromosone(mutated, score_config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutate1() {
        let (_, config) = configuration();
        let mut r = MockRando::new();
        let m = Mutate1::new();
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
