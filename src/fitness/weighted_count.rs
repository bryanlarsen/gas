use super::FitnessFunction;
use crate::chromosone::{self, Chromosone};

/**

If each symbol has preferred locations in the chromosone, this may be expressed using the `WeightedCount` fitness score.   Every symbol has a weight for each position in the chromosone.   The weight is a very small positive integer indicating the desirability of the position.  The higher the weight, the more scores that increase when a symbol is placed in that position.

*/
pub struct WeightedCount {
    pub max_weight: usize,
    pub weights: [[usize; chromosone::LENGTH]; chromosone::NSYMS],
}

impl WeightedCount {
    /// see [`WeightedCount`].   `max_weight` is the maximum value for a weight.   [`WeightedCount`] will return `max_weight` scores for each symbol, so you likely want to set `max_weight` to a small number, like 1 or 3.
    pub const fn new(
        max_weight: usize,
        weights: [[usize; chromosone::LENGTH]; chromosone::NSYMS],
    ) -> WeightedCount {
        WeightedCount {
            max_weight,
            weights,
        }
    }
}

impl FitnessFunction for WeightedCount {
    fn nscores(&self) -> usize {
        self.max_weight * chromosone::NSYMS
    }

    fn weights(&self) -> Vec<f64> {
        return vec![1.0; self.nscores()];
    }

    fn run(&self, chromosone: &Chromosone) -> Vec<f64> {
        let mut scores: Vec<f64> = vec![0f64; self.nscores()];

        for i in 0..chromosone.len() {
            for w in 0..self.weights[chromosone[i] as usize][i] {
                scores[chromosone[i] as usize * self.max_weight + w as usize] += 1.0;
            }
        }

        scores
    }

    fn describe(&self, chromosone: &Chromosone) -> Vec<String> {
        let mut descriptions = Vec::<String>::with_capacity(chromosone::NSYMS);
        for g in 0..chromosone::NSYMS {
            let mut count = 0usize;
            let mut sum = 0usize;
            for i in 0..chromosone.len() {
                if chromosone[i] as usize == g {
                    count += 1;
                    sum += self.weights[g][i];
                }
            }
            let mean = (sum as f64) / (count as f64);
            descriptions.push(format!("{:.2}", mean));
        }
        descriptions
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weighted() {
        let wc = WeightedCount::new(2, [[2, 2, 1, 1, 0], [1, 1, 2, 2, 0], [2, 2, 2, 2, 2]]);
        let scores = wc.run(&[0, 0, 0, 1, 1]);
        assert_eq!(scores, vec![3.0, 2.0, 1.0, 1.0, 0.0, 0.0]);
    }
}
