use crate::config::*;
use crate::constraints;
use crate::crossover;
use crate::fitness;
use crate::game;
use crate::mutation;
use crate::tournaments;

use crate::schedule_data;

// schedule_data.rs is automatically generated, hoist symbols
pub const NSYMS: usize = schedule_data::NSYMS;
pub const LENGTH: usize = schedule_data::LENGTH;
pub const NCOLORS: usize = schedule_data::NCOLORS;
pub const MAX_WEIGHT: usize = schedule_data::MAX_WEIGHT;

pub const POPSIZE: usize = 100;
pub const NSCORES: usize = /* distance */
    NSYMS * 3 + /* color */ NCOLORS * NSYMS + /* weighted */ MAX_WEIGHT * NSYMS;

// specifies the configuration of how to create a generation.  Each generation is built from tournament winners and offspring, this specifies how much of each and how they are configured.
pub fn configuration() -> Configuration {
    Configuration::new(InitConfiguration {
        tournament: Box::new(tournaments::scale::Scale::new(
            tournaments::single_elimination::SingleElimination::new(game::full::Full::new()),
            1,
            1.0,
            2.0,
        )),
        crossover: vec![
            (Box::new(crossover::null::Null::new()), 2),
            (Box::new(crossover::splice::Splice::new()), 1),
            (Box::new(crossover::mix::Mix::new()), 1),
        ],
        mutation: vec![
            (Box::new(mutation::null::Null::new()), 10),
            (Box::new(mutation::mutate::Mutate::new(1)), 1),
            (Box::new(mutation::mutate::Mutate::new(2)), 1),
            (Box::new(mutation::mutate::Mutate::new(3)), 1),
            (Box::new(mutation::rotate::Rotate::new(1)), 1),
            (Box::new(mutation::rotate::Rotate::new(2)), 1),
            (Box::new(mutation::rotate::Rotate::new(3)), 1),
        ],
        fitness: vec![
            Box::new(fitness::distance::Distance::new(7)),
            Box::new(fitness::color_count::ColorCount::new(
                schedule_data::CHROMOSONE_COLORS,
                schedule_data::COLOR_PREFS,
            )),
            Box::new(fitness::weighted_count::WeightedCount::new(
                schedule_data::MAX_WEIGHT,
                schedule_data::WEIGHTS,
            )),
        ],
        constraint: vec![Box::new(
            constraints::invalid_position::InvalidPosition::new(schedule_data::INVALID_POSITIONS),
        )],
    })
}
