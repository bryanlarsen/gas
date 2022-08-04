use crate::config::*;
use crate::constraints;
use crate::fitness;
use crate::game;
use crate::reproduction;
use crate::tournaments;

use crate::schedule_data;

// schedule_data.rs is automatically generated, hoist symbols
pub const NSYMS: usize = schedule_data::NSYMS;
pub const LENGTH: usize = schedule_data::LENGTH;
pub const NCOLORS: usize = schedule_data::NCOLORS;
pub const MAX_WEIGHT: usize = schedule_data::MAX_WEIGHT;

pub const POPSIZE: usize = 1000;
pub const TRIES_PER_GAME: std::ops::Range<usize> = 125..500;
pub const NSCORES: usize = /* distance */
    NSYMS * 2 + /* color */ NCOLORS * NSYMS + /* weighted */ MAX_WEIGHT * NSYMS;

// specifies the configuration of how to create a generation.  Each generation is built from tournament winners and offspring, this specifies how much of each and how they are configured.   the "n" values must sum to POPSIZE.
pub fn configuration() -> Configuration {
    let config = Configuration {
        generation: vec![
            GenerationConfig {
                n: 500,
                propagation: Propagation::Tournament(Box::new(
                    tournaments::single_elimination::SingleElimination::new(
                        game::sample::Sample::new(TRIES_PER_GAME),
                    ),
                )),
            },
            GenerationConfig {
                n: 100,
                propagation: Propagation::Crossover(Box::new(reproduction::splice::Splice::new())),
            },
            GenerationConfig {
                n: 100,
                propagation: Propagation::Crossover(Box::new(reproduction::mix::Mix::new())),
            },
            GenerationConfig {
                n: 50,
                propagation: Propagation::Mutation(Box::new(reproduction::mutate::Mutate::new(1))),
            },
            GenerationConfig {
                n: 50,
                propagation: Propagation::Mutation(Box::new(reproduction::mutate::Mutate::new(2))),
            },
            GenerationConfig {
                n: 50,
                propagation: Propagation::Mutation(Box::new(reproduction::mutate::Mutate::new(3))),
            },
            GenerationConfig {
                n: 50,
                propagation: Propagation::Mutation(Box::new(reproduction::rotate::Rotate::new(1))),
            },
            GenerationConfig {
                n: 50,
                propagation: Propagation::Mutation(Box::new(reproduction::rotate::Rotate::new(2))),
            },
            GenerationConfig {
                n: 50,
                propagation: Propagation::Mutation(Box::new(reproduction::rotate::Rotate::new(3))),
            },
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
        iteration: 0,
    };

    assert_eq!(
        config.generation.iter().fold(0, |sum, c| sum + c.n),
        POPSIZE
    );

    config
}
