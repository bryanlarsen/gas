use crate::chromosone::Chromosone;

pub mod invalid_position;

/**

Constraints are used to filter out nonviable chromosone's.   In other words, a candidate without a constraint violation will always beat a candidate with one.   More specifically, the number of constraint violations is what is important.   If all candidates have constraint violations, one of the candidates with the fewest violations will win a tournament.

*/

pub trait Constraint {
    fn run(&self, chromosone: &Chromosone) -> usize;
}

pub struct ConstraintConfig {
    pub constraints: Vec<Box<dyn Constraint + Sync + Send>>,
}

impl ConstraintConfig {
    pub fn new(constraints: Vec<Box<dyn Constraint + Sync + Send>>) -> ConstraintConfig {
        ConstraintConfig { constraints }
    }

    pub fn violations(&self, chromosone: &Chromosone) -> usize {
        self.constraints
            .iter()
            .fold(0usize, |sum, cf| sum + cf.run(chromosone))
    }
}
