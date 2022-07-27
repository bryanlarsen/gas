#[cfg(not(test))]
use crate::data::*;
#[cfg(test)]
use crate::test_data::*;

pub mod invalid_position;

/**

Constraints are used to filter out inviable chromosone's.   In other words, a candidate without a constraint violation will always beat a candidate with one.   More specifically, the number of constraint violations is what is important.   If all candidates have constraint violations, one of the candidates with the fewest violations will win a tournament.

*/

pub trait Constraint {
    /// returns the number of violations
    fn run(&self, chromosone: &[usize; LENGTH]) -> usize;
}

pub fn constraint_violations(
    chromosone: &[usize; LENGTH],
    constraints: &[Box<dyn Constraint>],
) -> usize {
    constraints
        .iter()
        .fold(0usize, |sum, c| sum + c.run(chromosone))
}
