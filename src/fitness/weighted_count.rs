use super::{FitnessFunction, FitnessName};
use crate::chromosone::Gene;

/**

If each symbol has preferred locations in the chromosone, this may be expressed using the `WeightedCount` fitness score.   Every symbol has a weight for each position in the chromosone.   The weight is a very small positive integer indicating the desirability of the position.  The higher the weight, the more scores that increase when a symbol is placed in that position.

*/
pub struct WeightedCount<const N: usize, const NSYMS: usize> {
    pub max_weight: usize,
    pub weights: Vec<Vec<usize>>, //LENGTH; chromosone::NSYMS
}

impl<const N: usize, const NSYMS: usize> WeightedCount<N, NSYMS> {
    /// see [`WeightedCount`].   `max_weight` is the maximum value for a weight.   [`WeightedCount`] will return `max_weight` scores for each symbol, so you likely want to set `max_weight` to a small number, like 1 or 3.
    pub const fn new(max_weight: usize, weights: Vec<Vec<usize>>) -> WeightedCount<N, NSYMS> {
        WeightedCount {
            max_weight,
            weights,
        }
    }
}

impl<const N: usize, const NSYMS: usize> FitnessFunction<N, NSYMS> for WeightedCount<N, NSYMS> {
    fn nscores(&self) -> usize {
        self.max_weight * NSYMS
    }

    fn weights(&self) -> Vec<f64> {
        return vec![1.0; self.nscores()];
    }

    fn run(&self, chromosone: &[Gene; N]) -> Vec<f64> {
        let mut scores: Vec<f64> = vec![0f64; self.nscores()];

        for i in 0..chromosone.len() {
            for w in 0..self.weights[chromosone[i] as usize][i] {
                scores[chromosone[i] as usize * self.max_weight + w as usize] += 1.0;
            }
        }

        scores
    }

    fn names(&self) -> Vec<FitnessName> {
        let mut names: Vec<FitnessName> = Vec::with_capacity(self.nscores());
        for i in 0..NSYMS {
            for w in 0..self.max_weight {
                names.push(FitnessName {
                    prefix: format!("desirability{w}"),
                    gene: Some(i),
                    locus: None,
                });
            }
        }
        names
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weighted() {
        let wc = WeightedCount::<5, 3>::new(
            2,
            vec![
                vec![2, 2, 1, 1, 0],
                vec![1, 1, 2, 2, 0],
                vec![2, 2, 2, 2, 2],
            ],
        );
        let scores = wc.run(&[0, 0, 0, 1, 1]);
        assert_eq!(scores, vec![3.0, 2.0, 1.0, 1.0, 0.0, 0.0]);
        assert_eq!(wc.nscores(), scores.len());
    }
}
