use crate::config::{default::LENGTH, Config};

pub mod invalid_position;

/**

Constraints are used to filter out inviable chromosone's.   In other words, a candidate without a constraint violation will always beat a candidate with one.   More specifically, the number of constraint violations is what is important.   If all candidates have constraint violations, one of the candidates with the fewest violations will win a tournament.

*/

pub enum Constraint {
    #[cfg_attr(test, allow(dead_code))]
    InvalidPosition(invalid_position::InvalidPosition),
}

impl Constraint {
    /// returns the number of violations
    pub fn run(&self, chromosone: &[usize; LENGTH]) -> usize {
        match self {
            Constraint::InvalidPosition(c) => c.run(chromosone),
        }
    }

    pub fn violations(chromosone: &[usize; LENGTH]) -> usize {
        Config::current()
            .constraints
            .constraints
            .iter()
            .fold(0usize, |sum, c| sum + c.run(chromosone))
    }
}

/// Create a const CONSTRAINT_CONFIG
pub struct ConstraintConfig {
    pub constraints: &'static [Constraint],
}

impl ConstraintConfig {
    pub const fn new(constraints: &'static [Constraint]) -> ConstraintConfig {
        ConstraintConfig { constraints }
    }
}
