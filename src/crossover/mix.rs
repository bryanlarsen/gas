use crate::config::config::LENGTH;

use crate::candidate::Candidate;

#[mockall_double::double]
use crate::rando::Rando;

pub struct Mix {}

impl Mix {
    pub const fn new() -> Self {
        Self {}
    }

    pub fn run(&self, left: &Candidate, right: &Candidate, rng: &mut Rando) -> Candidate {
        let mut child = left.chromosone.clone();
        let mut r = rng.uniform_iter(0..2);
        for i in 0..LENGTH {
            if r.next() == Some(0) {
                child[i] = right.chromosone[i];
            }
        }
        Candidate::from_chromosone(child)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::*;

    #[test]
    fn test_mix() {
        let mut r = Rando::default();
        let m = Mix::new();
        r.expect_uniform_iter()
            .with(predicate::eq(0..2))
            .times(1)
            .returning(|_| [1, 0, 1, 0, 1].iter().cloned());
        Candidate::assert_eq(
            &m.run(
                &Candidate::from_chromosone([0, 1, 2, 0, 1]),
                &Candidate::from_chromosone([2, 0, 0, 1, 2]),
                &mut r,
            ),
            &Candidate::from_chromosone([0, 0, 2, 1, 1]),
        );
    }
}
