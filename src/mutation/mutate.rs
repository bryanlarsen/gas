use crate::config::config::{LENGTH, NSYMS};

use crate::candidate::Candidate;

#[mockall_double::double]
use crate::rando::Rando;

#[cfg(test)]
use mockall::*;

pub struct Mutate {
    pub n: usize,
}

impl Mutate {
    pub const fn new(n: usize) -> Mutate {
        Mutate { n }
    }

    pub fn run(&self, candidate: &Candidate, rng: &mut Rando) -> Candidate {
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
        Candidate::from_chromosone(mutated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutate() {
        let mut r = Rando::default();
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
            &m.run(&Candidate::from_chromosone([0, 1, 2, 0, 1]), &mut r),
            &Candidate::from_chromosone([0, 2, 2, 0, 1]),
        );
    }
}
