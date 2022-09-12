use crate::chromosone::Gene;

pub mod color_count;
pub mod distance;
pub mod locus_desirability;
pub mod weighted_count;

/**
**  A FitnessFunction returns a set of fitness scores when passed a chromosone.
**
**  This is implemented as an enum rather than a trait so that we can store them
**  inside a const [FitnessConfig]
**
**  Fitness scores are floating point numbers where a higher number is considered
**  better. Negative numbers are allowed, so if you wish to optimize to zero you
**  may simply return the negative of the absolute value.
**
**  NaN values are valid scores, and indicate that the score cannot be
**  calculated. When comparing scores, any number compared to a NaN is considered
**  a tie.
*
**  Implementations: [color_count::ColorCount], [distance::Distance], [weighted_count::WeightedCount]
***/

pub trait FitnessFunction<const N: usize, const NSYMS: usize> {
    /// returns a vector of floats where bigger numbers are better.   If your fitness function optimizes to 0, remember that 0 is the biggest negative number.  NaN is also a valid score, and means that the score cannot be compared and is considered a tie with any other number.
    fn run(&self, chromosone: &[Gene; N]) -> Vec<f64>;
    /// provides an [FitnessFunction.nscores] length human readable name for the scores returned
    fn names(&self) -> Vec<FitnessName>;
    fn nscores(&self) -> usize;
    /// not all games take weights into account, but some do.  The vector must be [FitnessFunction.nscores] long.  Scores with higher weights are worth `weight` times as much as nominal scores.   Note that the value of the score is irrelevant -- scores are never compared with scores in a different position, scores are only compared with the same score on a different candidate.   The weight signifies how valuable it is that this particular score on one candidate is higher or lower than the same score on a different candidate.
    fn weights(&self) -> Vec<f64>;
}

/// FitnessName describes a score.   The full name would be [FitnessName.prefix] + [FitnessName.gene] name + [FitnessName.locus] name
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FitnessName {
    pub prefix: String,
    /// If has a value, the score is specific to the gene, otherwise applies to all genes
    pub gene: Option<usize>,
    /// If has a value, the score is specific to the locus, otherwise applies to all locuses
    pub locus: Option<usize>,
}

pub struct FitnessConfig<const N: usize, const NSYMS: usize> {
    pub functions: Vec<Box<dyn FitnessFunction<N, NSYMS> + Sync + Send>>,
    pub nscores: usize,
}

impl<const N: usize, const NSYMS: usize> FitnessConfig<N, NSYMS> {
    pub fn new(
        functions: Vec<Box<dyn FitnessFunction<N, NSYMS> + Sync + Send>>,
    ) -> FitnessConfig<N, NSYMS> {
        let nscores = FitnessConfig::nscores(&functions);
        FitnessConfig { functions, nscores }
    }

    /// helper function, use [FitnessConfig::new]
    fn nscores(functions: &Vec<Box<dyn FitnessFunction<N, NSYMS> + Sync + Send>>) -> usize {
        // this used to be a const fn, that's why we don't use for or iter.
        let mut sum = 0usize;
        let mut i = 0usize;
        while i < functions.len() {
            sum += functions[i].nscores();
            i += 1;
        }
        sum
    }

    pub fn scores(&self, chromosone: &[Gene; N]) -> Vec<f64> {
        let mut scores = Vec::<f64>::with_capacity(self.nscores);
        for func in self.functions.iter() {
            scores.append(&mut func.run(chromosone));
        }
        scores
    }

    pub fn weights(&self) -> Vec<f64> {
        self.functions.iter().fold(
            Vec::<f64>::with_capacity(self.nscores),
            |mut weights, func| {
                weights.extend_from_slice(&func.weights());
                weights
            },
        )
    }

    pub fn names(&self) -> Vec<FitnessName> {
        self.functions.iter().fold(
            Vec::<FitnessName>::with_capacity(self.nscores),
            |mut names, func| {
                names.extend_from_slice(&func.names());
                names
            },
        )
    }
}

#[cfg(test)]
/// cannot use assert_eq! on scores because they sometimes contain NaN.  NaN != NaN, which is true mathematically, but sucks in unit tests
pub fn assert_scores_eq(left: &[f64], right: &[f64]) {
    assert_eq!(left.len(), right.len(), "left {:?} right {:?}", left, right);
    for i in 0..left.len() {
        if !left[i].is_nan() || !right[i].is_nan() {
            assert_eq!(left[i], right[i], "left {:?} right {:?}", left, right);
        }
    }
}

impl FitnessName {
    pub fn to_string(&self, symbol_names: &[&str], locus_names: &[&str]) -> String {
        format!(
            "{} {} {}",
            self.prefix,
            if let Some(g) = self.gene {
                symbol_names[g]
            } else {
                ""
            },
            if let Some(i) = self.locus {
                locus_names[i]
            } else {
                ""
            }
        )
    }
}
