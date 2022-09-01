use super::Mutation;
use crate::chromosone::Chromosone;

#[cfg(test)]
use mockall::*;

#[mockall_double::double]
use crate::rando::Rando;
/**
*   If `n` is 1 then this mutator swaps the genes at 2 locuses.   If `n` is 2 or more, then this mutator chooses n+1 locuses, and rotates the genes through those positions.
**/
pub struct Rotate {
    pub n: usize,
}

impl Rotate {
    pub const fn new(n: usize) -> Rotate {
        Rotate { n }
    }
}

impl Mutation for Rotate {
    fn run(&self, chromosone: &Chromosone, rng: &mut Rando) -> Chromosone {
        let mut mutated = chromosone.clone();
        let mut curpos = rng.gen_range(0..chromosone.len());
        let origval = chromosone[curpos];
        let mut nextpos = curpos;
        for _ in 0..self.n {
            nextpos = rng.gen_range(0..chromosone.len());
            mutated[curpos] = chromosone[nextpos];
            curpos = nextpos;
        }
        mutated[nextpos] = origval;
        mutated
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chromosone;

    #[test]
    fn test_rotate1() {
        let mut r = Rando::default();
        let m = Rotate::new(1);
        r.expect_gen_range()
            .with(predicate::eq(0..chromosone::LENGTH))
            .times(1)
            .return_const(1usize);
        r.expect_gen_range()
            .with(predicate::eq(0..chromosone::LENGTH))
            .times(1)
            .return_const(3usize);
        assert_eq!(m.run(&[0, 1, 2, 0, 1], &mut r), [0, 0, 2, 1, 1]);
    }

    #[test]
    fn test_rotate2() {
        let mut r = Rando::default();
        let m = Rotate::new(2);
        r.expect_gen_range()
            .with(predicate::eq(0..chromosone::LENGTH))
            .times(1)
            .return_const(0usize);
        r.expect_gen_range()
            .with(predicate::eq(0..chromosone::LENGTH))
            .times(1)
            .return_const(1usize);
        r.expect_gen_range()
            .with(predicate::eq(0..chromosone::LENGTH))
            .times(1)
            .return_const(2usize);
        assert_eq!(m.run(&[0, 1, 2, 0, 1], &mut r), [1, 2, 0, 0, 1]);
    }
}
