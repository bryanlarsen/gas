#!/usr/bin/env bash

set -eo pipefail
set -x

template() {
    cat <<EOF > src/data.rs

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

pub const POPSIZE: usize = ${POPSIZE};
pub const TRIES_PER_GAME: std::ops::Range<usize> = ${TRIES_PER_GAME_MIN}..${TRIES_PER_GAME_MAX};
pub const NSCORES: usize = /* distance */
    NSYMS * 2 + /* color */ NCOLORS * NSYMS + /* weighted */ MAX_WEIGHT * NSYMS;

// specifies the configuration of how to create a generation.  Each generation is built from tournament winners and offspring, this specifies how much of each and how they are configured.   the "n" values must sum to POPSIZE.
pub fn configuration() -> Configuration {
    let config = Configuration {
        generation: vec![
            GenerationConfig {
                n: ${SINGLE_ELIMINATION},
                propagation: Propagation::Tournament(Box::new(
                    tournaments::single_elimination::SingleElimination::new(
                        game::sample::Sample::new(TRIES_PER_GAME),
                    ),
                )),
            },
            GenerationConfig {
                n: $SPLICE,
                propagation: Propagation::Crossover(Box::new(reproduction::splice::Splice::new())),
            },
            GenerationConfig {
                n: $MIX,
                propagation: Propagation::Crossover(Box::new(reproduction::mix::Mix::new())),
            },
            GenerationConfig {
                n: $MUTATION1,
                propagation: Propagation::Mutation(Box::new(reproduction::mutate::Mutate::new(1))),
            },
            GenerationConfig {
                n: $MUTATION2,
                propagation: Propagation::Mutation(Box::new(reproduction::mutate::Mutate::new(2))),
            },
            GenerationConfig {
                n: $MUTATION3,
                propagation: Propagation::Mutation(Box::new(reproduction::mutate::Mutate::new(3))),
            },
            GenerationConfig {
                n: $ROTATE1,
                propagation: Propagation::Mutation(Box::new(reproduction::rotate::Rotate::new(1))),
            },
            GenerationConfig {
                n: $ROTATE2,
                propagation: Propagation::Mutation(Box::new(reproduction::rotate::Rotate::new(2))),
            },
            GenerationConfig {
                n: $ROTATE3,
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

EOF
}

sample () {
    template
    cargo build
    for i in $(seq ${N}) ; do
        iteration
    done
}

iteration () {
    local json=$(cargo run -- ${ITERATIONS})
    ELAPSED_MS=$(jq .elapsed_ms <<< ${json})
    VIOLATIONS=$(jq .violations <<< ${json})
    SCORE=$(jq .score <<< ${json})
    ITERATION=$(jq .iteration <<< ${json})
    AVERAGE_ITERATION=$(jq .average_iteration <<< ${json})

    echo ${SCORE},${VIOLATIONS},${ELAPSED_MS},${ITERATIONS},${POPSIZE},${SINGLE_ELIMINATION},${MUTATION1},${TRIES_PER_GAME_MIN},${TRIES_PER_GAME_MAX},${MUTATION2},${MUTATION3},${ROTATE1},${ROTATE2},${ROTATE3},${SPLICE},${MIX},${ITERATION},${AVERAGE_ITERATION} | tee >> results.csv
}

main () {
    TRIES_PER_GAME_MIN=125
    TRIES_PER_GAME_MAX=500
    N=1
    for i in $(seq 6) ; do
    for pop in 60 100 200 500 1000 ; do
        POPSIZE=${pop}
        ITERATIONS=$((2000000 / pop))
        SINGLE_ELIMINATION=$((pop / 2))
        MUTATION1=$((pop / 20))
        MUTATION2=$((pop / 20))
        MUTATION3=$((pop / 20))
        ROTATE1=$((pop / 20))
        ROTATE2=$((pop / 20))
        ROTATE3=$((pop / 20))
        SPLICE=$((pop / 10))
        MIX=$((pop / 10))
        sample
    done
    done
}

main "$@"
