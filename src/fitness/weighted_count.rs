use crate::config::config::{LENGTH, NSYMS};

/**

If each symbol has preferred locations in the chromosone, this may be expressed using the `WeightedCount` fitness score.   Every symbol has a weight for each position in the chromosone.   The weight is a very small positive integer indicating the desirability of the position.  The higher the weight, the more scores that increase when a symbol is placed in that position.

*/
pub struct WeightedCount {
    pub max_weight: usize,
    pub weights: [[usize; LENGTH]; NSYMS],
}

impl WeightedCount {
    /// see [`WeightedCount`].   `max_weight` is the maximum value for a weight.   [`WeightedCount`] will return `max_weight` scores for each symbol, so you likely want to set `max_weight` to a small number, like 1 or 3.
    pub const fn new(max_weight: usize, weights: [[usize; LENGTH]; NSYMS]) -> WeightedCount {
        WeightedCount {
            max_weight,
            weights,
        }
    }

    pub const fn nscores(&self) -> usize {
        self.max_weight * NSYMS
    }

    pub fn run(&self, chromosone: &[usize]) -> Vec<f64> {
        let mut scores: Vec<f64> = vec![0f64; NSYMS * self.max_weight];

        for i in 0..chromosone.len() {
            for w in 0..self.weights[chromosone[i]][i] {
                scores[chromosone[i] * self.max_weight + w] += 1.0;
            }
        }

        scores
    }

    pub fn describe(&self, chromosone: &[usize]) -> Vec<String> {
        let mut descriptions = Vec::<String>::with_capacity(NSYMS);
        for g in 0..NSYMS {
            let mut count = 0usize;
            let mut sum = 0usize;
            for i in 0..chromosone.len() {
                if chromosone[i] == g {
                    count += 1;
                    sum += self.weights[g][i];
                }
            }
            let mean = (sum as f64) / (count as f64);
            descriptions.push(mean.to_string());
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
