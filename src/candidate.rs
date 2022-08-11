use crate::constraints::constraint_violations;
#[cfg(not(test))]
use crate::data::*;
#[cfg(test)]
use crate::test_data::*;

use crate::config::Configuration;
use crate::fitness::*;

#[cfg(test)]
use mockall::*;

#[cfg_attr(test, mockall_double::double)]
use crate::rando::Rando;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Candidate {
    pub chromosone: [usize; LENGTH],
    pub scores: [f64; NSCORES],
    pub violations: usize,
    pub iteration: usize,
}

impl Candidate {
    pub fn from_chromosone(chromosone: [usize; LENGTH], config: &Configuration) -> Candidate {
        Candidate {
            chromosone,
            scores: scores(&chromosone, &config.fitness),
            violations: constraint_violations(&chromosone, &config.constraint),
            iteration: config.iteration,
        }
    }

    pub fn new(config: &Configuration, rng: &mut Rando) -> Candidate {
        Candidate::from_chromosone(array_init::array_init(|_| rng.gen_range(0..NSYMS)), config)
    }

    /// calculate an aggregate score.  The system doesn't use this internally, but it can be used for a very rough comparison between candidates.
    pub fn total_score(&self) -> f64 {
        self.scores.iter().filter(|s| !s.is_nan()).sum()
    }

    /// Hamming distance
    pub fn distance(&self, other: &Candidate) -> usize {
        let mut count = 0usize;
        for i in 0..self.chromosone.len() {
            if self.chromosone[i] != other.chromosone[i] {
                count += 1;
            }
        }
        count
    }

    /// give an estimate of a population's diversity where 1 == all the same and 0 == completely different.   Calculation is similar to a Hamming distance.
    pub fn similarity(population: &[Candidate]) -> f64 {
        let mut similarities = Vec::<f64>::with_capacity(population[0].chromosone.len());
        for i in 0..population[0].chromosone.len() {
            let mut map = std::collections::HashMap::<usize, usize>::new();
            let mut max_count = 0usize;
            for j in 0..population.len() {
                let count = match map.get_mut(&population[j].chromosone[i]) {
                    Some(v) => {
                        *v += 1;
                        *v
                    }
                    None => {
                        map.insert(population[j].chromosone[i], 1);
                        1usize
                    }
                };
                if count > max_count {
                    max_count = count;
                }
            }
            similarities.push((max_count - 1) as f64 / (population.len() - 1) as f64);
        }
        similarities.iter().sum::<f64>() / similarities.len() as f64
    }

    #[cfg(test)]
    /// cannot use assert_eq! on scores because they sometimes contain NaN.  NaN != NaN, which is true mathematically, but sucks in unit tests
    pub fn assert_eq(left: &Candidate, right: &Candidate) {
        assert_eq!(
            left.chromosone, right.chromosone,
            "left {:?} right {:?}",
            left, right
        );
        assert_eq!(
            left.violations, right.violations,
            "left {:?} right {:?}",
            left, right
        );
        assert_scores_eq(&left.scores, &right.scores);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candidate() {
        let config = configuration();
        Candidate::assert_eq(
            &Candidate::from_chromosone([0, 0, 1, 0, 1], &config),
            &Candidate {
                chromosone: [0, 0, 1, 0, 1],
                scores: [1.0, 1.5, 1.0, 2.0, 2.0, 2.0, f64::NAN, f64::NAN, 2.0],
                violations: 0,
                iteration: 0,
            },
        );
    }

    #[test]
    fn test_similarity() {
        let config = configuration();
        assert_eq!(
            Candidate::similarity(&[
                Candidate::from_chromosone([0usize, 1, 2, 0, 1], &config),
                Candidate::from_chromosone([0usize, 1, 2, 0, 1], &config),
                Candidate::from_chromosone([0usize, 1, 2, 0, 1], &config),
            ]),
            1.0
        );
        assert_eq!(
            Candidate::similarity(&[
                Candidate::from_chromosone([0usize, 1, 2, 0, 1], &config),
                Candidate::from_chromosone([1usize, 2, 0, 1, 2], &config),
                Candidate::from_chromosone([2usize, 0, 1, 2, 0], &config),
            ]),
            0.0
        );
        assert_eq!(
            Candidate::similarity(&[
                Candidate::from_chromosone([0usize, 1, 2, 0, 1], &config),
                Candidate::from_chromosone([1usize, 2, 0, 1, 2], &config),
                Candidate::from_chromosone([0usize, 1, 2, 0, 1], &config),
            ]),
            0.5
        );
        assert_eq!(
            Candidate::from_chromosone([0usize, 1, 2, 0, 1], &config)
                .distance(&Candidate::from_chromosone([1usize, 2, 0, 1, 2], &config)),
            5
        );
        assert_eq!(
            Candidate::from_chromosone([0usize, 1, 2, 0, 1], &config)
                .distance(&Candidate::from_chromosone([0usize, 1, 2, 1, 2], &config)),
            2
        );
    }

    #[test]
    fn test_new() {
        let config = configuration();
        let mut r = Rando::default();
        r.expect_gen_range()
            .with(predicate::eq(0..NSYMS))
            .times(LENGTH)
            .return_const(2usize);
        Candidate::assert_eq(
            &Candidate::new(&config, &mut r),
            &Candidate {
                chromosone: [2, 2, 2, 2, 2],
                scores: [
                    f64::NAN,
                    f64::NAN,
                    2.0,
                    f64::NAN,
                    f64::NAN,
                    2.0,
                    1.0,
                    1.0,
                    0.25,
                ],
                violations: 0,
                iteration: 0,
            },
        );
    }
}
