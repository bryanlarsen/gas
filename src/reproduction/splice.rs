#[cfg(not(test))]
use crate::data::*;
#[cfg(test)]
use crate::test_data::*;

use crate::candidate::Candidate;
use crate::config::Configuration;
use crate::reproduction::Crossover;

#[mockall_double::double]
use crate::rando::Rando;

pub struct Splice {}

impl Splice {
    pub fn new() -> Self {
        Self {}
    }
}

impl Crossover for Splice {
    fn run(
        &self,
        left: &Candidate,
        right: &Candidate,
        config: &Configuration,
        rng: &mut Rando,
    ) -> Candidate {
        let mut child = [0usize; LENGTH];
        let mut start;
        let mut end;
        loop {
            start = rng.gen_range(0..LENGTH);
            end = rng.gen_range(1..LENGTH + 1);
            if start > end {
                std::mem::swap(&mut start, &mut end);
            }
            if start != end {
                break;
            }
        }
        child[0..start].copy_from_slice(&left.chromosone[0..start]);
        child[start..end].copy_from_slice(&right.chromosone[start..end]);
        child[end..LENGTH].copy_from_slice(&left.chromosone[end..LENGTH]);
        Candidate::from_chromosone(child, config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::*;

    #[test]
    fn test_splice() {
        let config = configuration();
        let mut r = Rando::default();
        let m = Splice::new();
        r.expect_gen_range()
            .with(predicate::eq(0..LENGTH))
            .times(1)
            .return_const(1usize);
        r.expect_gen_range()
            .with(predicate::eq(1..LENGTH + 1))
            .times(1)
            .return_const(3usize);
        Candidate::assert_eq(
            &m.run(
                &Candidate::from_chromosone([0, 1, 2, 0, 1], &config),
                &Candidate::from_chromosone([2, 0, 0, 1, 2], &config),
                &config,
                &mut r,
            ),
            &Candidate::from_chromosone([0, 0, 0, 0, 1], &config),
        );
    }
}
