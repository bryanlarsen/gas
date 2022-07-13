#[cfg(not(test))]
use crate::data::*;
#[cfg(test)]
use crate::test_data::*;

use array_init::array_init;

use super::FitnessFunction;

/**
The Distance fitness scores discourage clumping of symbols in the chromosone and encourage identical symbols to spread out evenly.

It returns two scores for each symbol: the minimum distance between two instances of the same symbol in the chromosone, and the average distance.

If the symbol does not occur at least twice in the chromosone resulting in a distance that cannot be calculated, then the score is NaN.   Tournaments consider any comparison with NaN to be a tie so they are indifferent between any spacing and no spacing.
*/
pub struct Distance {
    pub max: usize,
}

impl Distance {
    /// creates a new [`Distance`].  `max` constrains resulting scores.  For instance in a scheduling system if you consider that as long as shifts are at least a week apart further spacing is not an improvement, you could set `max` to 7 (assuming there's only one shift per day in the chromoone).
    pub fn new(max: usize) -> Distance {
        Distance { max }
    }
}

impl FitnessFunction for Distance {
    fn run(&self, chromosone: &[usize; LENGTH]) -> Vec<f64> {
        let mut scores: Vec<f64> = Vec::with_capacity(NSYMS * 2);

        let mut current_position: Vec<usize> = vec![usize::MAX; NSYMS];
        let mut distances: [Vec<usize>; NSYMS] = array_init(|_| vec![]);

        //calculate distances
        for n in 0..chromosone.len() {
            if current_position[chromosone[n]] != usize::MAX {
                distances[chromosone[n]].push(n - current_position[chromosone[n]]);
            }
            current_position[chromosone[n]] = n;
        }

        for n in 0..NSYMS {
            if distances[n].len() > 0 {
                let minimum = distances[n].iter().min().unwrap();
                scores.push(std::cmp::min(self.max, *minimum) as f64);
                let average =
                    (distances[n].iter().sum::<usize>() as f64) / (distances[n].len() as f64);
                scores.push(if average > self.max as f64 {
                    self.max as f64
                } else {
                    average
                });
            } else {
                scores.push(f64::NAN);
                scores.push(f64::NAN);
            }
        }

        scores
    }
}

#[cfg(test)]
mod tests {
    use crate::fitness::assert_scores_eq;

    use super::*;

    #[test]
    fn test_distance() {
        let d = Distance::new(7);
        assert_scores_eq(
            &d.run(&[0, 0, 1, 0, 1]),
            &[1.0, 1.5, 2.0, 2.0, f64::NAN, f64::NAN],
        );
    }
    #[test]
    fn test_max() {
        let d = Distance::new(1);
        assert_scores_eq(
            &d.run(&[0, 0, 1, 0, 1]),
            &[1.0, 1.0, 1.0, 1.0, f64::NAN, f64::NAN],
        );
    }
}
