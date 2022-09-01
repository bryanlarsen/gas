pub mod color_count;
pub mod distance;
pub mod weighted_count;

use crate::chromosone::Chromosone;

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

pub trait FitnessFunction {
    /// returns a vector of floats where bigger numbers are better.   If your fitness function works better with 0 better, remember that 0 is the biggest negative number.  NaN is also a valid score, and means that the score cannot be compared and is considered a tie with any other number.
    fn run(&self, chromosone: &Chromosone) -> Vec<f64>;
    /// helpful for tuning but not necessary, so feel free to return an empty vector
    fn describe(&self, chromosone: &Chromosone) -> Vec<String>;
    fn nscores(&self) -> usize;
    /// not all games take weights into account, but some do.  Weights are small positive numbers, and the vector must be [nscores()] long.
    fn weights(&self) -> Vec<f64>;
}

pub struct FitnessConfig {
    pub functions: Vec<Box<dyn FitnessFunction + Sync + Send>>,
    pub nscores: usize,
}

impl FitnessConfig {
    pub fn new(functions: Vec<Box<dyn FitnessFunction + Sync + Send>>) -> FitnessConfig {
        let nscores = FitnessConfig::nscores(&functions);
        FitnessConfig { functions, nscores }
    }

    /// helper function, use [FitnessConfig::new]
    fn nscores(functions: &Vec<Box<dyn FitnessFunction + Sync + Send>>) -> usize {
        // this used to be a const fn, that's why we don't use for or iter.
        let mut sum = 0usize;
        let mut i = 0usize;
        while i < functions.len() {
            sum += functions[i].nscores();
            i += 1;
        }
        sum
    }

    pub fn scores(&self, chromosone: &Chromosone) -> Vec<f64> {
        let mut scores = Vec::<f64>::with_capacity(self.nscores);
        for func in self.functions.iter() {
            scores.append(&mut func.run(chromosone));
        }
        scores
    }

    pub fn describe(&self, chromosone: &Chromosone) -> Vec<Vec<String>> {
        let mut descriptions = Vec::<Vec<String>>::new();
        for func in self.functions.iter() {
            descriptions.push(func.describe(chromosone));
        }
        descriptions
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
