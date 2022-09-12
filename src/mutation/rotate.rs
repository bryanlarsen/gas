use super::Mutation;
use crate::chromosone::Gene;

#[cfg(test)]
use mockall::*;

#[mockall_double::double]
use crate::rando::Rando;
/**
*   If `n` is 1 then this mutator swaps the genes at 2 locuses.   If `n` is 2 or more, then this mutator chooses n+1 locuses, and rotates the genes through those positions.
**/
pub struct Rotate<const N: usize, const NSYMS: usize> {
    pub n: usize,
}

impl<const N: usize, const NSYMS: usize> Rotate<N, NSYMS> {
    pub const fn new(n: usize) -> Rotate<N, NSYMS> {
        Rotate { n }
    }
}

impl<const N: usize, const NSYMS: usize> Mutation<N, NSYMS> for Rotate<N, NSYMS> {
    fn run(&self, chromosone: &[Gene; N], rng: &mut Rando) -> [Gene; N] {
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

    #[test]
    fn test_rotate1() {
        let mut r = Rando::default();
        let m = Rotate::<5, 3>::new(1);
        r.expect_gen_range()
            .with(predicate::eq(0..5))
            .times(1)
            .return_const(1usize);
        r.expect_gen_range()
            .with(predicate::eq(0..5))
            .times(1)
            .return_const(3usize);
        assert_eq!(m.run(&[0, 1, 2, 0, 1], &mut r), [0, 0, 2, 1, 1]);
    }

    #[test]
    fn test_rotate2() {
        let mut r = Rando::default();
        let m = Rotate::<5, 3>::new(2);
        r.expect_gen_range()
            .with(predicate::eq(0..5))
            .times(1)
            .return_const(0usize);
        r.expect_gen_range()
            .with(predicate::eq(0..5))
            .times(1)
            .return_const(1usize);
        r.expect_gen_range()
            .with(predicate::eq(0..5))
            .times(1)
            .return_const(2usize);
        assert_eq!(m.run(&[0, 1, 2, 0, 1], &mut r), [1, 2, 0, 0, 1]);
    }
}
