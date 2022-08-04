use crate::config::*;
use crate::fitness::*;
use crate::game;
use crate::reproduction::*;
use crate::tournaments::*;

pub const NSYMS: usize = 3;
pub const LENGTH: usize = 5;
pub const TRIES_PER_GAME: std::ops::Range<usize> = 1usize..4usize;
pub const NSCORES: usize = NSYMS * 2;
pub const POPSIZE: usize = 2;
pub const NCOLORS: usize = 2;

/// the configuration for the creation of a generation.   The sum of "n" must add up to POPSIZE.
pub fn configuration() -> Configuration {
    Configuration {
        generation: vec![
            GenerationConfig {
                n: 1,
                propagation: Propagation::Tournament(Box::new(
                    single_elimination::SingleElimination::new(game::sample::Sample::new(
                        TRIES_PER_GAME,
                    )),
                )),
            },
            GenerationConfig {
                n: 1,
                propagation: Propagation::Mutation(Box::new(mutate::Mutate::new(1))),
            },
        ],
        fitness: vec![Box::new(distance::Distance::new(7))],
        constraint: vec![],
        iteration: 0,
    }
}
