use super::Mutation;
use crate::chromosone::{self, Chromosone};

#[mockall_double::double]
use crate::rando::Rando;

#[cfg(test)]
use mockall::*;

/**
*  This mutator randomly changes `n` locuses in the chromosone.
**/
pub struct Mutate {
    pub n: usize,
}

impl Mutate {
    pub const fn new(n: usize) -> Mutate {
        Mutate { n }
    }
}

impl Mutation for Mutate {
    fn run(&self, chromosone: &Chromosone, rng: &mut Rando) -> Chromosone {
        let mut mutated = chromosone.clone();
        for _ in 0..self.n {
            let pos = rng.gen_range(0..chromosone::LENGTH);
            let mut new;
            loop {
                new = rng.gen_range(0..chromosone::NSYMS) as chromosone::Gene;
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
        let m = Mutate::new(1);
        r.expect_gen_range()
            .with(predicate::eq(0..chromosone::LENGTH))
            .times(1)
            .return_const(1usize);
        r.expect_gen_range()
            .with(predicate::eq(0..chromosone::NSYMS))
            .times(1)
            .return_const(2usize);
        assert_eq!(m.run(&[0, 1, 2, 0, 1], &mut r), [0, 2, 2, 0, 1])
    }
}
