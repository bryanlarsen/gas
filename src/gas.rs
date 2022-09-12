pub mod cycle;
pub mod generation;

use crate::constraints::ConstraintConfig;
use crate::crossover::CrossoverConfig;
use crate::fitness::FitnessConfig;
use crate::mutation::MutationConfig;
use crate::tournaments::Tournament;

/// see module documentation
pub struct Gas<const N: usize, const NSYMS: usize> {
    /// the set of fitness functions that turn a chromosone into a set of fitness scores
    pub fitness: FitnessConfig<N, NSYMS>,
    /// constraints determine whether chromosones are valid or invalid
    pub constraints: ConstraintConfig<N, NSYMS>,
    /// crossovers and constraints are the heart of a genetic algorithm.
    pub crossovers: CrossoverConfig<N, NSYMS>,
    /// crossovers and constraints are the heart of a genetic algorithm.
    pub mutations: MutationConfig<N, NSYMS>,
    /// this is the tournament used in the algorithm, so is typically called millions of times.   faster, less accurate tournaments may therefore provide better results due to their speedup.
    pub cycle_tournament: Box<dyn Tournament<N, NSYMS> + Send + Sync>,
    /// used at the end of a cycle, a comprehensive tournament is best
    pub final_tournament: Box<dyn Tournament<N, NSYMS> + Send + Sync>,
    /// to ensure genetic diversity, the hamming distance between any two chromosones in the population must be at least this value
    pub taboo_distance: usize,
    pub population_size: usize,
}

#[cfg(test)]
impl Gas<5, 3> {
    /// device under test
    pub fn dut() -> Gas<5, 3> {
        use crate::fitness::distance::Distance;
        use crate::game::full::Full;
        use crate::mutation::mutate::Mutate;
        use crate::tournaments::single_elimination::SingleElimination;

        Gas {
            fitness: FitnessConfig::new(vec![Box::new(Distance::<5, 3>::new(
                7, [None; 3], [None; 3], 1.0, 1.0,
            ))]),
            constraints: ConstraintConfig::new(vec![]),
            cycle_tournament: Box::new(SingleElimination::new(Full::new())),
            final_tournament: Box::new(SingleElimination::new(Full::new())),
            crossovers: CrossoverConfig::new(vec![(
                1,
                Box::new(crate::crossover::null::Null::new()),
            )]),
            mutations: MutationConfig::new(vec![(1, Box::new(Mutate::<5, 3>::new(1)))]),
            taboo_distance: 1,
            population_size: 3,
        }
    }
}
