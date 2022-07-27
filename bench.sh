#!/usr/bin/env bash

set -eo pipefail
set -x

template() {
    cat <<EOF > src/data.rs
use crate::config::*;
use crate::constraints::*;
use crate::fitness::*;
use crate::game;
use crate::reproduction::*;
use crate::tournaments::*;

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
                    single_elimination::SingleElimination::new(Box::new(
                        game::sample::Sample::new(TRIES_PER_GAME),
                    )),
                )),
            },
            GenerationConfig {
                n: ${MUTATION},
                propagation: Propagation::Mutation(Box::new(mutate::Mutate1::new())),
            },
        ],
        fitness: vec![
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
        constraint: vec![Box::new(invalid_position::InvalidPosition::new(
            schedule_data::INVALID_POSITIONS,
        ))],
    };

    assert_eq!(config.generation.iter().fold(0, |sum, c| sum + c.n), POPSIZE);

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

    echo ${SCORE},${VIOLATIONS},${ELAPSED_MS},${ITERATIONS},${POPSIZE},${SINGLE_ELIMINATION},${MUTATION},${TRIES_PER_GAME_MIN},${TRIES_PER_GAME_MAX} | tee >> results.csv
}

main () {
    POPSIZE=60
    SINGLE_ELIMINATION=30
    MUTATION=30
    TRIES_PER_GAME_MIN=125
    TRIES_PER_GAME_MAX=500
    N=10
    for iter in 100 1000 2000 3000 5000 25000 100000 ; do
        ITERATIONS=${iter}
        sample
    done
}

main "$@"
