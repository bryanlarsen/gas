#[cfg(not(test))]
use crate::data::*;
#[cfg(test)]
use crate::test_data::*;

use crate::fitness::*;
use crate::rando::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Candidate {
    pub chromosone: [usize; LENGTH],
    pub scores: [f64; NSCORES],
}

impl Candidate {
    pub fn from_chromosone(
        chromosone: [usize; LENGTH],
        score_config: &[Box<dyn FitnessFunction>],
    ) -> Candidate {
        Candidate {
            chromosone,
            scores: scores(&chromosone, score_config),
        }
    }

    pub fn new(score_config: &[Box<dyn FitnessFunction>], rng: &mut dyn Rando) -> Candidate {
        let chromosone: [usize; LENGTH] = array_init::array_init(|_| rng.gen_range(0..NSYMS));
        Candidate {
            chromosone,
            scores: scores(&chromosone, score_config),
        }
    }

    /// calculate an aggregate score.  The system doesn't use this internally, but it can be used for a very rough comparison between candidates.
    #[cfg(not(test))]
    pub fn total_score(&self) -> f64 {
        self.scores.iter().filter(|s| !s.is_nan()).sum()
    }

    #[cfg(test)]
    /// cannot use assert_eq! on scores because they sometimes contain NaN.  NaN != NaN, which is true mathematically, but sucks in unit tests
    pub fn assert_eq(left: &Candidate, right: &Candidate) {
        assert_eq!(
            left.chromosone, right.chromosone,
            "left {:?} right {:?}",
            left, right
        );
        assert_scores_eq(&left.scores, &right.scores);
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_candidate() {
        let (_, config) = configuration();
        Candidate::assert_eq(
            &Candidate::from_chromosone([0, 0, 1, 0, 1], &config),
            &Candidate {
                chromosone: [0, 0, 1, 0, 1],
                scores: [1.0, 1.5, 2.0, 2.0, f64::NAN, f64::NAN],
            },
        );
    }

    #[test]
    fn test_new() {
        let (_, config) = configuration();
        let mut r = MockRando::new();
        r.expect_gen_range()
            .with(predicate::eq(0..NSYMS))
            .times(LENGTH)
            .return_const(2usize);
        Candidate::assert_eq(
            &Candidate::new(&config, &mut r),
            &Candidate {
                chromosone: [2, 2, 2, 2, 2],
                scores: [f64::NAN, f64::NAN, f64::NAN, f64::NAN, 1.0, 1.0],
            },
        );
    }
}
