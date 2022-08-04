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

#[cfg(not(test))]
pub struct RandoIter {
    pub iter: rand::distributions::DistIter<rand::distributions::Uniform<usize>, ThreadRng, usize>,
}

#[cfg(test)]
pub struct RandoIter {
    pub iter: std::iter::Cloned<std::slice::Iter<'static, usize>>,
}

// #[cfg(test)]
// impl Iterator for RandoIter {
//     type Item = usize;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.iter.next() {
//             Some(u) => Some(*u),
//             None => None,
//         }
//     }
// }

impl Iterator for RandoIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

#[cfg(test)]
impl RandoIter {
    pub fn mock(slice: &'static [usize]) -> Self {
        Self {
            iter: slice.iter().cloned(),
        }
    }
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
    pub fn uniform_iter(&self, range: std::ops::Range<usize>) -> RandoIter {
        RandoIter {
            iter: rand::distributions::Uniform::from(range).sample_iter(self.rng.clone()),
        }
    }

    #[cfg(test)]
    pub fn uniform_iter(&self, _range: std::ops::Range<usize>) -> RandoIter {
        RandoIter {
            iter: [0usize].iter().cloned(),
        }
    }
}
