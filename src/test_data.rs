use crate::config::*;
use crate::fitness::*;
use crate::reproduction::*;
use crate::tournaments::*;

pub const NSYMS: usize = 3;
pub const LENGTH: usize = 5;
pub const TRIES_PER_GAME: std::ops::Range<usize> = 1usize..4usize;
pub const NSCORES: usize = NSYMS * 2;
pub const POPSIZE: usize = 2;
pub const NCOLORS: usize = 2;

/// the configuration for the creation of a generation.   The sum of "n" must add up to POPSIZE.
pub fn configuration() -> ([ConfigLine; 2], [Box<dyn FitnessFunction>; 1]) {
    (
        [
            ConfigLine {
                n: 1,
                propagation: Propagation::Tournament(Box::new(
                    single_elimination::SingleElimination::new(),
                )),
            },
            ConfigLine {
                n: 1,
                propagation: Propagation::Mutation(Box::new(mutate::Mutate1::new())),
            },
        ],
        [Box::new(distance::Distance::new(7))],
    )
}
