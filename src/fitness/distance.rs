use super::{FitnessFunction, FitnessName};
use crate::chromosone::{self, Chromosone};

use array_init::array_init;

/**
The Distance fitness scores discourage clumping of symbols in the chromosone and encourage identical symbols to spread out evenly.

Two scores per gene are returned.  The second score is the negated standard deviation of the distances.  The standard deviation is negated since we optimize to larger numbers and we wish to optimize the standard deviation to zero.

The first score is designed to discourage clumping.  There are two parts to this score: the integer part and the fractional part.   The integral part is the minimum distance.   The fractional part is the inverse of the number of distances at the minimum.   If the minimum distance is greater than the `max` parameter, `max+1` is returned as the clumping score.

The first two scores returned are the stdev and clumping score for gene 0, the next two for gene 1, et cetera.

The parameters `distance_before` and `distance_after` are useful when the chromosone is a window into a continuous set of values.   Set them to the distance each gene is from the front and the end of the chromosone, respectively.   For example, if gene 2 previously expressed immediately preceding the chromosone window, `distance_before[2]` would have a value of 1.    To specify that the symbols are not present before or after the chromosone, use the flag value `usize::MAX`.

If the symbol does not occur at least twice in the chromosone (along with distance_before and distance_after) resulting in a distance that cannot be calculated, then the score is NaN.   Tournaments consider any comparison with NaN to be a tie so they are indifferent between any spacing and no spacing.
*/
pub struct Distance {
    pub max: usize,
    pub distance_before: [usize; chromosone::NSYMS],
    pub distance_after: [usize; chromosone::NSYMS],
    pub weight_minimum: f64,
    pub weight_stdev: f64,
}

impl Distance {
    /// creates a new [`Distance`].  `max` constrains resulting scores.  For instance in a scheduling system if you consider that as long as shifts are at least a week apart further spacing is not an improvement, you could set `max` to 7 (assuming there's only one shift per day in the chromoone).   The weights are used in the [`FitnessFunction.weights`] function -- 1.0 is a reasonable value for both.
    pub const fn new(
        max: usize,
        distance_before: [usize; chromosone::NSYMS],
        distance_after: [usize; chromosone::NSYMS],
        weight_minimum: f64,
        weight_stdev: f64,
    ) -> Distance {
        Distance {
            max,
            distance_before,
            distance_after,
            weight_minimum,
            weight_stdev,
        }
    }

    fn distances(&self, chromosone: &Chromosone) -> [Vec<usize>; chromosone::NSYMS] {
        let mut current_position: Vec<usize> = vec![usize::MAX; chromosone::NSYMS];
        let mut distances: [Vec<usize>; chromosone::NSYMS] = array_init(|_| vec![]);

        //calculate distances
        for pos in 0..chromosone.len() {
            let g: usize = chromosone[pos].into();
            if current_position[g] == usize::MAX {
                if self.distance_before[g] != usize::MAX {
                    distances[g].push(pos + self.distance_before[g]);
                }
            } else {
                distances[g].push(pos - current_position[g]);
            }
            current_position[g] = pos;
        }

        for g in 0..chromosone::NSYMS {
            if self.distance_after[g] != usize::MAX {
                if current_position[g] == usize::MAX {
                    if self.distance_before[g] != usize::MAX {
                        distances[g].push(
                            self.distance_after[g]
                                + (chromosone.len() - 1)
                                + self.distance_before[g],
                        );
                    }
                } else {
                    distances[g].push(
                        self.distance_after[g] + (chromosone.len() - current_position[g] - 1),
                    );
                }
            }
        }
        distances
    }
}

impl FitnessFunction for Distance {
    fn nscores(&self) -> usize {
        2 * chromosone::NSYMS
    }

    fn weights(&self) -> Vec<f64> {
        let mut weights = Vec::<f64>::with_capacity(self.nscores());
        for _ in 0..chromosone::NSYMS {
            weights.push(self.weight_minimum);
            weights.push(self.weight_stdev);
        }
        weights
    }

    fn run(&self, chromosone: &Chromosone) -> Vec<f64> {
        let mut scores: Vec<f64> = Vec::with_capacity(chromosone::NSYMS * 2);
        let distances = self.distances(chromosone);
        for g in 0..chromosone::NSYMS {
            if distances[g].len() > 0 {
                let minimum = distances[g].iter().min().unwrap();
                let min_count = distances[g].iter().fold(0usize, |count, d| {
                    if *d == *minimum {
                        count + 1
                    } else {
                        count
                    }
                });
                let score = *minimum as f64 + 1.0 / min_count as f64;
                scores.push(if *minimum > self.max {
                    1.0 + self.max as f64
                } else {
                    score
                });

                let sum = distances[g].iter().sum::<usize>() as f64;
                let mean = sum / (distances[g].len() as f64);
                let stdev = distances[g]
                    .iter()
                    .map(|distance| {
                        let diff = mean - (*distance as f64);
                        diff * diff
                    })
                    .sum::<f64>()
                    / distances[g].len() as f64;
                scores.push(-stdev);
            } else {
                scores.push(f64::NAN);
                scores.push(f64::NAN);
            }
        }
        scores
    }

    fn names(&self) -> Vec<FitnessName> {
        let mut names = Vec::<FitnessName>::with_capacity(self.nscores());
        for g in 0..chromosone::NSYMS {
            names.push(FitnessName {
                prefix: "min distance".to_string(),
                gene: Some(g),
                locus: None,
            });
            names.push(FitnessName {
                prefix: "distance std dev".to_string(),
                gene: Some(g),
                locus: None,
            });
        }
        names
    }
    /*
    fn describe(&self, chromosone: &Chromosone) -> Vec<String> {
        let mut descriptions = Vec::<String>::with_capacity(chromosone::NSYMS);
        let scores = self.run(chromosone);
        let distances = self.distances(chromosone);
        for g in 0..chromosone::NSYMS {
            if distances[g].len() > 0 {
                let minimum = distances[g].iter().min().unwrap();
                let min_count = distances[g].iter().fold(0usize, |count, d| {
                    if *d == *minimum {
                        count + 1
                    } else {
                        count
                    }
                });
                let stdev = scores[g * 2 + 1];
                descriptions.push(format!("{}x{}u, stdev: {:.2}", min_count, minimum, stdev));
            } else {
                descriptions.push("".to_string());
            }
        }
        descriptions
    }
    */
}

#[cfg(test)]
mod tests {
    use crate::fitness::assert_scores_eq;

    use super::*;

    #[test]
    fn test_distance() {
        let d = Distance::new(7, [usize::MAX; 3], [usize::MAX; 3], 1.0, 1.0);
        let scores = d.run(&[0, 0, 1, 0, 1]);
        assert_scores_eq(
            &scores,
            &[
                1.0 + 1.0 / 1.0,
                -(0.5 * 0.5 + 0.5 * 0.5) / 2.0,
                2.0 + 1.0 / 1.0,
                -(0.0 * 0.0) / 1.0,
                f64::NAN,
                f64::NAN,
            ],
        );
        assert_eq!(d.nscores(), scores.len());
    }
    #[test]
    fn test_max() {
        let d = Distance::new(1, [usize::MAX; 3], [usize::MAX; 3], 1.0, 1.0);
        assert_scores_eq(
            &d.run(&[0, 0, 1, 0, 1]),
            &[
                2.0,
                -(0.5 * 0.5 + 0.5 * 0.5) / 2.0,
                2.0,
                -(0.0 * 0.0) / 1.0,
                f64::NAN,
                f64::NAN,
            ],
        );
    }
    #[test]
    fn test_before_after() {
        let d = Distance::new(99, [1, 2, 9], [1, 3, 9], 1.0, 1.0);
        assert_scores_eq(
            &d.run(&[0, 0, 1, 0, 1]),
            &[
                // gene0 has d of 1, 1, 2, 2.   Avg is 1.5
                1.0 + 1.0 / 2.0,
                -(4.0 * 0.5 * 0.5) / 4.0,
                // gene1 has d of 4, 2, 3.  Avg is 3..
                2.0 + 1.0 / 1.0,
                -(1.0 * 1.0 + 1.0 * 1.0 + 0.0 * 0.0) / 3.0,
                // gene2 has d of 22
                22.0 + 1.0 / 1.0,
                -0.0 / 1.0,
            ],
        );
    }
}
