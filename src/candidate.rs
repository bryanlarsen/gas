use crate::chromosone::{self, Chromosone};
use crate::gas::Gas;

#[cfg_attr(test, mockall_double::double)]
use crate::rando::Rando;

#[derive(Debug, PartialEq, Clone)]
pub struct Candidate {
    pub chromosone: Chromosone,
    pub scores: Vec<f64>,
    pub violations: usize,
}

impl Candidate {
    pub fn from_chromosone(gas: &Gas, chromosone: Chromosone) -> Candidate {
        Candidate {
            chromosone,
            scores: gas.fitness.scores(&chromosone),
            violations: gas.constraints.violations(&chromosone),
        }
    }

    pub fn new(gas: &Gas, rng: &mut Rando) -> Candidate {
        Candidate::from_chromosone(
            gas,
            array_init::array_init(|_| rng.gen_range(0..chromosone::NSYMS) as chromosone::Gene),
        )
    }

    #[cfg_attr(test, allow(dead_code))]
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

    #[allow(dead_code)]
    /// give an estimate of a population's diversity where 1 == all the same and 0 == completely different.   Calculation is similar to a Hamming distance.
    pub fn similarity(population: &[Candidate]) -> f64 {
        let mut similarities = Vec::<f64>::with_capacity(population[0].chromosone.len());
        for i in 0..population[0].chromosone.len() {
            let mut map = std::collections::HashMap::<usize, usize>::new();
            let mut max_count = 0usize;
            for j in 0..population.len() {
                let count = match map.get_mut(&population[j].chromosone[i].into()) {
                    Some(v) => {
                        *v += 1;
                        *v
                    }
                    None => {
                        map.insert(population[j].chromosone[i].into(), 1);
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
        crate::fitness::assert_scores_eq(&left.scores, &right.scores);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::*;

    #[test]
    fn test_candidate() {
        let gas = Gas::dut();
        Candidate::assert_eq(
            &Candidate::from_chromosone(&gas, [0, 0, 1, 0, 1]),
            &Candidate {
                chromosone: [0, 0, 1, 0, 1],
                scores: gas.fitness.scores(&[0, 0, 1, 0, 1]),
                violations: 0,
            },
        );
    }

    #[test]
    fn test_similarity() {
        let gas = Gas::dut();
        assert_eq!(
            Candidate::similarity(&[
                Candidate::from_chromosone(&gas, [0, 1, 2, 0, 1]),
                Candidate::from_chromosone(&gas, [0, 1, 2, 0, 1]),
                Candidate::from_chromosone(&gas, [0, 1, 2, 0, 1]),
            ]),
            1.0
        );
        assert_eq!(
            Candidate::similarity(&[
                Candidate::from_chromosone(&gas, [0, 1, 2, 0, 1]),
                Candidate::from_chromosone(&gas, [1, 2, 0, 1, 2]),
                Candidate::from_chromosone(&gas, [2, 0, 1, 2, 0]),
            ]),
            0.0
        );
        assert_eq!(
            Candidate::similarity(&[
                Candidate::from_chromosone(&gas, [0, 1, 2, 0, 1]),
                Candidate::from_chromosone(&gas, [1, 2, 0, 1, 2]),
                Candidate::from_chromosone(&gas, [0, 1, 2, 0, 1]),
            ]),
            0.5
        );
        assert_eq!(
            Candidate::from_chromosone(&gas, [0, 1, 2, 0, 1])
                .distance(&Candidate::from_chromosone(&gas, [1, 2, 0, 1, 2])),
            5
        );
        assert_eq!(
            Candidate::from_chromosone(&gas, [0, 1, 2, 0, 1])
                .distance(&Candidate::from_chromosone(&gas, [0, 1, 2, 1, 2])),
            2
        );
    }

    #[test]
    fn test_new() {
        let gas = Gas::dut();
        let mut r = Rando::default();
        r.expect_gen_range()
            .with(predicate::eq(0..chromosone::NSYMS))
            .times(chromosone::LENGTH)
            .return_const(2usize);
        Candidate::assert_eq(
            &Candidate::new(&gas, &mut r),
            &Candidate {
                chromosone: [2, 2, 2, 2, 2],
                scores: gas.fitness.scores(&[2, 2, 2, 2, 2]),
                violations: 0,
            },
        );
    }
}
