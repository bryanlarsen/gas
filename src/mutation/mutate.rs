use super::Mutation;
use crate::chromosone::Gene;

#[mockall_double::double]
use crate::rando::Rando;

#[cfg(test)]
use mockall::*;

/**
*  This mutator randomly changes `n` locuses in the chromosone.
**/
pub struct Mutate<const N: usize, const NSYMS: usize> {
    pub n: usize,
}

impl<const N: usize, const NSYMS: usize> Mutate<N, NSYMS> {
    pub const fn new(n: usize) -> Self {
        Mutate { n }
    }
}

impl<const N: usize, const NSYMS: usize> Mutation<N, NSYMS> for Mutate<N, NSYMS> {
    fn run(&self, chromosone: &[Gene; N], rng: &mut Rando) -> [Gene; N] {
        let mut mutated = chromosone.clone();
        for _ in 0..self.n {
            let pos = rng.gen_range(0..N);
            let mut new;
            loop {
                new = rng.gen_range(0..NSYMS) as Gene;
                if new != chromosone[pos] {
                    break;
                }
            }
            mutated[pos] = new;
        }
        mutated
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutate() {
        let mut r = Rando::default();
        let m = Mutate::<5, 3>::new(1);
        r.expect_gen_range()
            .with(predicate::eq(0..5))
            .times(1)
            .return_const(1usize);
        r.expect_gen_range()
            .with(predicate::eq(0..3))
            .times(1)
            .return_const(2usize);
        assert_eq!(m.run(&[0, 1, 2, 0, 1], &mut r), [0, 2, 2, 0, 1])
    }
}
