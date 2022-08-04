#[cfg(not(test))]
use crate::data::*;
#[cfg(test)]
use crate::test_data::*;

use crate::candidate::Candidate;
use crate::config::Configuration;
use crate::reproduction::Crossover;

#[mockall_double::double]
use crate::rando::Rando;

pub struct Mix {}

impl Mix {
    pub fn new() -> Self {
        Self {}
    }
}

impl Crossover for Mix {
    fn run(
        &self,
        left: &Candidate,
        right: &Candidate,
        config: &Configuration,
        rng: &mut Rando,
    ) -> Candidate {
        let mut child = left.chromosone.clone();
        let mut r = rng.uniform_iter(0..2);
        for i in 0..LENGTH {
            if r.next() == Some(0) {
                child[i] = right.chromosone[i];
            }
        }
        Candidate::from_chromosone(child, config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::*;

    #[test]
    fn test_mix() {
        let config = configuration();
        let mut r = Rando::default();
        let m = Mix::new();
        r.expect_uniform_iter()
            .with(predicate::eq(0..2))
            .times(1)
            .returning(|_| [1, 0, 1, 0, 1].iter().cloned());
        Candidate::assert_eq(
            &m.run(
                &Candidate::from_chromosone([0, 1, 2, 0, 1], &config),
                &Candidate::from_chromosone([2, 0, 0, 1, 2], &config),
                &config,
                &mut r,
            ),
            &Candidate::from_chromosone([0, 0, 2, 1, 1], &config),
        );
    }
}
