use crate::chromosone::Gene;
use crate::crossover::Crossover;

#[mockall_double::double]
use crate::rando::Rando;

/** Mix is a crossover function that mixes the two crossovers on a locus-by-locus basis rather than a splice.

To use a deck of cards analogy, it's a riff rather than a cut **/
pub struct Mix<const N: usize, const NSYMS: usize> {}

impl<const N: usize, const NSYMS: usize> Mix<N, NSYMS> {
    pub const fn new() -> Self {
        Self {}
    }
}

impl<const N: usize, const NSYMS: usize> Crossover<N, NSYMS> for Mix<N, NSYMS> {
    fn run(&self, left: &[Gene; N], right: &[Gene; N], rng: &mut Rando) -> [Gene; N] {
        let mut child = left.clone();
        let mut r = rng.uniform_iter(0..2);
        for i in 0..left.len() {
            if r.next() == Some(0) {
                child[i] = right[i];
            }
        }
        child
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::*;

    #[test]
    fn test_mix() {
        let mut r = Rando::default();
        let m = Mix::<5, 3>::new();
        r.expect_uniform_iter()
            .with(predicate::eq(0..2))
            .times(1)
            .returning(|_| [1, 0, 1, 0, 1].iter().cloned());
        assert_eq!(
            m.run(&[0, 1, 2, 0, 1], &[2, 0, 0, 1, 2], &mut r),
            [0, 0, 2, 1, 1]
        );
    }
}
