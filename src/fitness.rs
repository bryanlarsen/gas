pub mod color_count;
pub mod distance;
pub mod weighted_count;

#[cfg(not(test))]
use crate::data::*;
#[cfg(test)]
use crate::test_data::*;

pub trait FitnessFunction {
    fn run(&self, chromosone: &[usize; LENGTH]) -> Vec<f64>;
}

pub fn scores(
    chromosone: &[usize; LENGTH],
    score_config: &[Box<dyn FitnessFunction>],
) -> [f64; NSCORES] {
    let mut scores = [0f64; NSCORES];
    let mut i = 0;
    for func in score_config {
        let s = func.run(chromosone);
        scores[i..i + s.len()].copy_from_slice(&s);
        i += s.len();
    }
    scores
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
