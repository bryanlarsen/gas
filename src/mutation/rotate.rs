use crate::config::default::LENGTH;

use crate::candidate::Candidate;

#[cfg(test)]
use mockall::*;

#[mockall_double::double]
use crate::rando::Rando;

pub struct Rotate {
    pub n: usize,
}

impl Rotate {
    pub const fn new(n: usize) -> Rotate {
        Rotate { n }
    }

    pub fn run(&self, candidate: &Candidate, rng: &mut Rando) -> Candidate {
        let mut mutated = [0usize; LENGTH];
        mutated.copy_from_slice(&candidate.chromosone);
        let mut curpos = rng.gen_range(0..LENGTH);
        let origval = candidate.chromosone[curpos];
        let mut nextpos = curpos;
        for _ in 0..self.n {
            nextpos = rng.gen_range(0..LENGTH);
            mutated[curpos] = candidate.chromosone[nextpos];
            curpos = nextpos;
        }
        mutated[nextpos] = origval;
        Candidate::from_chromosone(mutated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate1() {
        let mut r = Rando::default();
        let m = Rotate::new(1);
        r.expect_gen_range()
            .with(predicate::eq(0..LENGTH))
            .times(1)
            .return_const(1usize);
        r.expect_gen_range()
            .with(predicate::eq(0..LENGTH))
            .times(1)
            .return_const(3usize);
        Candidate::assert_eq(
            &m.run(&Candidate::from_chromosone([0, 1, 2, 0, 1]), &mut r),
            &Candidate::from_chromosone([0, 0, 2, 1, 1]),
        );
    }

    #[test]
    fn test_rotate2() {
        let mut r = Rando::default();
        let m = Rotate::new(2);
        r.expect_gen_range()
            .with(predicate::eq(0..LENGTH))
            .times(1)
            .return_const(0usize);
        r.expect_gen_range()
            .with(predicate::eq(0..LENGTH))
            .times(1)
            .return_const(1usize);
        r.expect_gen_range()
            .with(predicate::eq(0..LENGTH))
            .times(1)
            .return_const(2usize);
        Candidate::assert_eq(
            &m.run(&Candidate::from_chromosone([0, 1, 2, 0, 1]), &mut r),
            &Candidate::from_chromosone([1, 2, 0, 0, 1]),
        );
    }
}
