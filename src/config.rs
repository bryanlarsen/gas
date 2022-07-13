use crate::reproduction::*;
use crate::tournaments::*;

pub enum Propagation {
    Tournament(Box<dyn Tournament>),
    Mutation(Box<dyn Mutation>),
    //    Breeding(Breeding),
}

/// the configuration for a portion of a generation.   If propogation is a tournament, a single tournament is held and the top "n" winners are added to the next generation.   If propagation is a reproduction, then "n" reproductions are done and added to the generation.

pub struct ConfigLine {
    pub n: usize,
    pub propagation: Propagation,
}
