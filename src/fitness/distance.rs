#[cfg(not(test))]
use crate::data::*;
#[cfg(test)]
use crate::test_data::*;

use array_init::array_init;

use super::FitnessFunction;

/**
The Distance fitness scores discourage clumping of symbols in the chromosone and encourage identical symbols to spread out evenly.

It returns three scores for each symbol: the minimum distance between two instances of the same symbol in the chromosone, the average distance and the inverse of the count of adjacent identical symbols (returning 2 if no adjacent symbols).

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
        let mut scores: Vec<f64> = Vec::with_capacity(NSYMS * 3);

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
                let sum = distances[n]
                    .iter()
                    .map(|d| usize::min(self.max, *d))
                    .sum::<usize>() as f64;
                scores.push(sum / (distances[n].len() as f64));
                let zcount = distances[n]
                    .iter()
                    .fold(0usize, |c, d| if *d == 1 { c + 1 } else { c });
                scores.push(if zcount == 0 {
                    2.0
                } else {
                    1.0 / zcount as f64
                });
            } else {
                scores.push(f64::NAN);
                scores.push(f64::NAN);
                scores.push(2.0);
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
            &[1.0, 1.5, 1.0, 2.0, 2.0, 2.0, f64::NAN, f64::NAN, 2.0],
        );
    }
    #[test]
    fn test_max() {
        let d = Distance::new(1);
        assert_scores_eq(
            &d.run(&[0, 0, 1, 0, 1]),
            &[1.0, 1.0, 1.0, 1.0, 1.0, 2.0, f64::NAN, f64::NAN, 2.0],
        );
    }
}
