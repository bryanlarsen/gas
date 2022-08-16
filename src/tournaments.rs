pub mod scale;
pub mod single_elimination;

use crate::candidate::Candidate;

#[mockall_double::double]
use crate::rando::Rando;

/** A tournament ranks candidates.   It returns a winner plus a ranking for each candidate in the population, with higher numbers being better.
 */
pub trait Tournament {
    fn run(&self, population: &Vec<Candidate>, rng: &mut Rando) -> (Candidate, Vec<usize>);
}
