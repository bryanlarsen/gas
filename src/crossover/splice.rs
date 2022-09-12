use crate::chromosone::Gene;
use crate::crossover::Crossover;

#[mockall_double::double]
use crate::rando::Rando;

/** If each chromosone is a deck of cards, splice would cut each deck twice in the same place, and then use the middle from one deck and the top & bottom from the other. **/
pub struct Splice<const N: usize, const NSYMS: usize> {}

impl<const N: usize, const NSYMS: usize> Splice<N, NSYMS> {
    pub const fn new() -> Self {
        Self {}
    }
}

impl<const N: usize, const NSYMS: usize> Crossover<N, NSYMS> for Splice<N, NSYMS> {
    fn run(&self, left: &[Gene; N], right: &[Gene; N], rng: &mut Rando) -> [Gene; N] {
        let mut child = left.clone();
        let mut start;
        let mut end;
        loop {
            start = rng.gen_range(0..child.len());
            end = rng.gen_range(1..child.len() + 1);
            if start != end {
                break;
            }
        }
        if start > end {
            child[..start].copy_from_slice(&right[..start]);
            child[end..].copy_from_slice(&right[end..]);
        } else {
            child[start..end].copy_from_slice(&right[start..end]);
        }
        child
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::*;

    #[test]
    fn test_splice() {
        let mut r = Rando::default();
        let m = Splice::<5, 3>::new();
        r.expect_gen_range()
            .with(predicate::eq(0..5))
            .times(1)
            .return_const(1usize);
        r.expect_gen_range()
            .with(predicate::eq(1..6))
            .times(1)
            .return_const(3usize);
        assert_eq!(
            m.run(&[0, 1, 2, 0, 1], &[2, 0, 0, 1, 2], &mut r),
            [0, 0, 0, 0, 1]
        );
    }
}
