use crate::config::*;
use crate::fitness::*;
use crate::reproduction::*;
use crate::tournaments::*;

use crate::schedule_data;

// schedule_data.rs is automatically generated, hoist symbols
pub const NSYMS: usize = schedule_data::NSYMS;
pub const LENGTH: usize = schedule_data::LENGTH;
pub const NCOLORS: usize = schedule_data::NCOLORS;
pub const MAX_WEIGHT: usize = schedule_data::MAX_WEIGHT;
pub const POPSIZE: usize = schedule_data::POPSIZE;

pub const TRIES_PER_GAME: std::ops::Range<usize> = LENGTH / 2..LENGTH * 2;
pub const NSCORES: usize = /* distance */
    NSYMS * 2 + /* color */ NCOLORS * NSYMS + /* weighted */ MAX_WEIGHT * NSYMS;

// specifies the configuration of how to create a generation.  Each generation is built from tournament winners and offspring, this specifies how much of each and how they are configured.   the "n" values must sum to POPSIZE.
pub fn configuration() -> ([ConfigLine; 2], [Box<dyn FitnessFunction>; 3]) {
    (
        [
            ConfigLine {
                n: 20,
                propagation: Propagation::Tournament(Box::new(
                    single_elimination::SingleElimination::new(),
                )),
            },
            ConfigLine {
                n: 20,
                propagation: Propagation::Mutation(Box::new(mutate::Mutate1::new())),
            },
        ],
        [
            Box::new(distance::Distance::new(7)),
            Box::new(color_count::ColorCount::new(
                schedule_data::CHROMOSONE_COLORS,
                schedule_data::COLOR_PREFS,
            )),
            Box::new(weighted_count::WeightedCount::new(
                schedule_data::MAX_WEIGHT,
                schedule_data::WEIGHTS,
            )),
        ],
    )
}
