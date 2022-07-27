use crate::constraints::Constraint;
use crate::fitness::FitnessFunction;
use crate::reproduction::Mutation;
use crate::tournaments::Tournament;

pub enum Propagation {
    Tournament(Box<dyn Tournament>),
    Mutation(Box<dyn Mutation>),
    //    Breeding(Breeding),
}

/// the configuration for a portion of a generation.   If propogation is a tournament, a single tournament is held and the top "n" winners are added to the next generation.   If propagation is a reproduction, then "n" reproductions are done and added to the generation.

pub struct GenerationConfig {
    pub n: usize,
    pub propagation: Propagation,
}

pub struct Configuration {
    pub generation: Vec<GenerationConfig>,
    pub fitness: Vec<Box<dyn FitnessFunction>>,
    pub constraint: Vec<Box<dyn Constraint>>,
}
