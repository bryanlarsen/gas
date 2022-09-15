pub mod double_elimination;
pub mod elo;
pub mod full_season;
pub mod single_elimination;

pub use double_elimination::DoubleElimination;
pub use full_season::FullSeason;
pub use single_elimination::SingleElimination;

use crate::candidate::Candidate;

#[mockall_double::double]
use crate::rando::Rando;

/// A tournament ranks candidates. It returns a winner plus an elo ranking with
/// average 1000 for each candidate in the population.
pub trait Tournament<const N: usize, const NSYMS: usize> {
    fn run(
        &self,
        population: &Vec<Candidate<N, NSYMS>>,
        rng: &mut Rando,
        score_weights: &Vec<f64>,
    ) -> (Candidate<N, NSYMS>, Vec<usize>);
}
