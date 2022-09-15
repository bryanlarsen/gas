use crate::chromosone::Gene;

pub mod invalid_position;
pub use invalid_position::InvalidPosition;

/**

Constraints are used to filter out nonviable chromosone's.   In other words, a candidate without a constraint violation will always beat a candidate with one.   More specifically, the number of constraint violations is what is important.   If all candidates have constraint violations, one of the candidates with the fewest violations will win a tournament.

*/

pub trait Constraint<const N: usize, const NSYMS: usize> {
    fn run(&self, chromosone: &[Gene; N]) -> usize;
}

pub struct ConstraintConfig<const N: usize, const NSYMS: usize> {
    pub constraints: Vec<Box<dyn Constraint<N, NSYMS> + Sync + Send>>,
}

impl<const N: usize, const NSYMS: usize> ConstraintConfig<N, NSYMS> {
    pub fn new(
        constraints: Vec<Box<dyn Constraint<N, NSYMS> + Sync + Send>>,
    ) -> ConstraintConfig<N, NSYMS> {
        ConstraintConfig { constraints }
    }

    pub fn violations(&self, chromosone: &[Gene; N]) -> usize {
        self.constraints
            .iter()
            .fold(0usize, |sum, cf| sum + cf.run(chromosone))
    }
}
