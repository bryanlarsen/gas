pub mod single_elimination;

use crate::candidate::*;

#[mockall_double::double]
use crate::rando::Rando;

/** a tournament sorts candidates, with "better" candidates toward the end of
 * the list. The result is returned as an list of indices into the input
 * population vector.
 */
pub trait Tournament {
    fn run(&self, population: &Vec<Candidate>, rng: &mut Rando) -> Vec<usize>;
}
