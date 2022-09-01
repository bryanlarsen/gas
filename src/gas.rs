pub mod cycle;
pub mod generation;

use crate::constraints::ConstraintConfig;
use crate::crossover::CrossoverConfig;
use crate::fitness::FitnessConfig;
use crate::mutation::MutationConfig;
use crate::tournaments::Tournament;

/// see module documentation
pub struct Gas {
    pub fitness: FitnessConfig,
    pub constraints: ConstraintConfig,
    pub cycle_tournament: Box<dyn Tournament + Send + Sync>,
    pub final_tournament: Box<dyn Tournament + Send + Sync>,
    pub crossovers: CrossoverConfig,
    pub mutations: MutationConfig,
    pub taboo_distance: usize,
    pub population_size: usize,
}

#[cfg(test)]
impl Gas {
    /// device under test
    pub fn dut() -> Gas {
        use crate::fitness::distance::Distance;
        use crate::game::full::Full;
        use crate::mutation::mutate::Mutate;
        use crate::tournaments::single_elimination::SingleElimination;

        Gas {
            fitness: FitnessConfig::new(vec![Box::new(Distance::new(
                7,
                [usize::MAX; 3],
                [usize::MAX; 3],
                1.0,
                1.0,
            ))]),
            constraints: ConstraintConfig::new(vec![]),
            cycle_tournament: Box::new(SingleElimination::new(Full::new())),
            final_tournament: Box::new(SingleElimination::new(Full::new())),
            crossovers: CrossoverConfig::new(vec![(
                1,
                Box::new(crate::crossover::null::Null::new()),
            )]),
            mutations: MutationConfig::new(vec![(1, Box::new(Mutate::new(1)))]),
            taboo_distance: 1,
            population_size: 3,
        }
    }
}
