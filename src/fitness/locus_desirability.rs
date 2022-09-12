use super::{FitnessFunction, FitnessName};
use crate::chromosone::Gene;

/**

If each symbol has preferred locations in the chromosone, this may be expressed using the `LocusDesirability` fitness score.   The input to this FitnessFunction is the score each symbol gives to each locus in the chromosone.   The fitness function simply reports the chosen symbol's score for each locus in the genome.

*/
pub struct LocusDesirability<const N: usize, const NSYMS: usize> {
    pub symbol_scores: Vec<Vec<f64>>, // chromosone::LENGTH; chromosone::NSYMS
    pub weight: f64,
}

impl<const N: usize, const NSYMS: usize> LocusDesirability<N, NSYMS> {
    pub const fn new(symbol_scores: Vec<Vec<f64>>, weight: f64) -> LocusDesirability<N, NSYMS> {
        LocusDesirability {
            symbol_scores,
            weight,
        }
    }
}

impl<const N: usize, const NSYMS: usize> FitnessFunction<N, NSYMS> for LocusDesirability<N, NSYMS> {
    fn nscores(&self) -> usize {
        N
    }

    fn weights(&self) -> Vec<f64> {
        return vec![self.weight; self.nscores()];
    }

    fn run(&self, chromosone: &[Gene; N]) -> Vec<f64> {
        chromosone
            .iter()
            .enumerate()
            .map(|(i, gene)| self.symbol_scores[*gene as usize][i])
            .collect()
    }

    fn names(&self) -> Vec<FitnessName> {
        (0..N)
            .map(|l| FitnessName {
                prefix: "".to_string(),
                gene: None,
                locus: Some(l),
            })
            .collect()
    }

    /*    fn describe(&self, chromosone: &Chromosone) -> Vec<String> {
        let mut descriptions = Vec::<String>::with_capacity(chromosone::NSYMS);
        for g in 0..chromosone::NSYMS {
            let mut count = 0f64;
            let mut sum = 0f64;
            for i in 0..chromosone.len() {
                if chromosone[i] as usize == g {
                    count += 1.0;
                    sum += self.symbol_scores[g][i];
                }
            }
            let mean = sum / count;
            descriptions.push(format!("{:.2}", mean));
        }
        descriptions
    }
    */
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locus_desirability() {
        let wc = LocusDesirability::<5, 3>::new(
            vec![
                vec![2.0, 2.0, 1.0, 1.0, 0.0],
                vec![1.0, 1.0, 2.0, 2.0, 0.0],
                vec![2.0, 2.0, 2.0, 2.0, 2.0],
            ],
            1.0,
        );
        let scores = wc.run(&[0, 0, 0, 1, 1]);
        assert_eq!(scores, vec![2.0, 2.0, 1.0, 2.0, 0.0]);
        assert_eq!(wc.nscores(), scores.len());
    }
}
