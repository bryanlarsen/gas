/*!

This module wraps some of the standard rand::* functions in a trait so that they can be mocked for testing.

 */
#[cfg(test)]
use mockall::*;

#[cfg(test)]
use crate::test_data::*;

#[cfg(not(test))]
use crate::data::*;

use rand::prelude::*;

#[cfg_attr(test, automock)]
pub trait Rando {
    fn shuffle(&mut self, s: &mut [usize]);
    fn gen_range(&mut self, range: std::ops::Range<usize>) -> usize;
    /// gen_range(0..LENGTH)
    fn gen_length(&mut self) -> usize;
    fn init_weighted(&mut self, weights: &[usize]);
    fn gen_weighted(&mut self) -> usize;
}

pub struct RealRando {
    pub rng: ThreadRng,
    pub weighted_distribution: Option<Box<rand::distributions::WeightedIndex<usize>>>,
    pub uniform_length: rand::distributions::Uniform<usize>,
}

impl RealRando {
    pub fn new() -> RealRando {
        RealRando {
            rng: thread_rng(),
            weighted_distribution: None,
            uniform_length: rand::distributions::Uniform::from(0..LENGTH),
        }
    }
}

impl Rando for RealRando {
    fn shuffle(&mut self, s: &mut [usize]) {
        s.shuffle(&mut self.rng)
    }

    fn gen_range(&mut self, range: std::ops::Range<usize>) -> usize {
        self.rng.gen_range(range)
    }

    fn gen_length(&mut self) -> usize {
        self.uniform_length.sample(&mut self.rng)
    }

    fn init_weighted(&mut self, weights: &[usize]) {
        self.weighted_distribution = Some(Box::new(
            rand::distributions::WeightedIndex::new(weights).unwrap(),
        ));
    }

    fn gen_weighted(&mut self) -> usize {
        self.weighted_distribution
            .as_ref()
            .unwrap()
            .sample(&mut self.rng)
    }
}
