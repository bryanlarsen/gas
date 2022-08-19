use crate::config::config::FITNESS_CONFIG;
pub mod color_count;
pub mod distance;
pub mod weighted_count;

/**
*  A FitnessFunction returns a set of fitness scores when passed a chromosone.
*
*  This is implemented as an enum rather than a trait so that we can store them
*  inside a const [FitnessConfig]
*
*  Fitness scores are floating point numbers where a higher number is considered
*  better. Negative numbers are allowed, so if you wish to optimize to zero you
*  may simply return the negative of the absolute value.
*
*  NaN values are valid scores, and indicate that the score cannot be
*  calculated. When comparing scores, any number compared to a NaN is considered
*  a tie.
*
*  Implementations: [color_count::ColorCount], [distance::Distance], [weighted_count::WeightedCount]
*/

pub enum FitnessFunction {
    #[cfg_attr(test, allow(dead_code))]
    ColorCount(color_count::ColorCount),
    Distance(distance::Distance),
    #[cfg_attr(test, allow(dead_code))]
    WeightedCount(weighted_count::WeightedCount),
}

/// Create a const FITNESS_CONFIG with [FitnessConfig::new]
pub struct FitnessConfig {
    pub functions: &'static [FitnessFunction],
    pub nscores: usize,
}

impl FitnessConfig {
    pub const fn new(functions: &'static [FitnessFunction]) -> FitnessConfig {
        FitnessConfig {
            functions,
            nscores: {
                // for and iter().fold() aren't yet usable in const fn
                let mut sum = 0usize;
                let mut i = 0usize;
                while i < functions.len() {
                    sum += functions[i].nscores();
                    i += 1;
                }
                sum
            },
        }
    }
}

impl FitnessFunction {
    pub fn run(&self, chromosone: &[usize]) -> Vec<f64> {
        match self {
            FitnessFunction::ColorCount(ff) => ff.run(chromosone),
            FitnessFunction::Distance(ff) => ff.run(chromosone),
            FitnessFunction::WeightedCount(ff) => ff.run(chromosone),
        }
    }

    pub const fn nscores(&self) -> usize {
        match self {
            FitnessFunction::ColorCount(ff) => ff.nscores(),
            FitnessFunction::Distance(ff) => ff.nscores(),
            FitnessFunction::WeightedCount(ff) => ff.nscores(),
        }
    }

    pub fn scores(chromosone: &[usize]) -> [f64; FITNESS_CONFIG.nscores] {
        let mut scores = [0f64; FITNESS_CONFIG.nscores];
        let mut i = 0;
        for func in FITNESS_CONFIG.functions {
            let s = func.run(chromosone);
            scores[i..i + s.len()].copy_from_slice(&s);
            i += s.len();
        }
        scores
    }

    pub fn describe(&self, chromosone: &[usize]) -> Vec<String> {
        match self {
            FitnessFunction::ColorCount(ff) => ff.describe(chromosone),
            FitnessFunction::Distance(ff) => ff.describe(chromosone),
            FitnessFunction::WeightedCount(ff) => ff.describe(chromosone),
        }
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
