/*!

This module wraps some of the standard rand::* functions in a trait so that they can be mocked for testing.

 */

#[cfg(test)]
use mockall::*;

use rand::prelude::*;

pub struct Rando {
    pub rng: ThreadRng,
    pub weighted_distribution: Option<Box<rand::distributions::WeightedIndex<usize>>>,
}

#[cfg_attr(test, automock)]
#[cfg_attr(test, allow(dead_code))]
impl Rando {
    pub fn new() -> Self {
        Self {
            rng: ThreadRng::default(),
            weighted_distribution: None,
        }
    }

    pub fn shuffle(&mut self, s: &mut [usize]) {
        s.shuffle(&mut self.rng)
    }

    pub fn gen_range(&mut self, range: std::ops::Range<usize>) -> usize {
        self.rng.gen_range(range)
    }

    #[cfg(not(test))]
    pub fn uniform_iter(
        &self,
        range: std::ops::Range<usize>,
    ) -> rand::distributions::DistIter<rand::distributions::Uniform<usize>, ThreadRng, usize> {
        rand::distributions::Uniform::from(range).sample_iter(self.rng.clone())
    }

    #[cfg(test)]
    pub fn uniform_iter(
        &self,
        _range: std::ops::Range<usize>,
    ) -> std::iter::Cloned<std::slice::Iter<'static, usize>> {
        [0usize].iter().cloned()
    }
}
