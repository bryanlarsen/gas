#!/usr/bin/env bash

set -eo pipefail
set -x

template() {
    cat <<EOF > src/data.rs
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

pub const POPSIZE: usize = ${POPSIZE};
pub const NSCORES: usize = /* distance */
    NSYMS * 3 + /* color */ NCOLORS * NSYMS + /* weighted */ MAX_WEIGHT * NSYMS;

// specifies the configuration of how to create a generation.  Each generation is built from tournament winners and offspring, this specifies how much of each and how they are configured.
pub fn configuration() -> Configuration {
    Configuration::new(InitConfiguration {
        tournament: Box::new(tournaments::scale::Scale::new(
            tournaments::single_elimination::SingleElimination::new(game::full::Full::new()),
            ${CONSTANT},
            ${SCALE},
            ${EXPONENT},
        )),
        crossover: vec![
            (Box::new(crossover::null::Null::new()), ${CROSSOVER_NULL}),
            (Box::new(crossover::splice::Splice::new()), ${SPLICE}),
            (Box::new(crossover::mix::Mix::new()), ${MIX}),
        ],
        mutation: vec![
            (Box::new(mutation::null::Null::new()), ${MUTATION_NULL}),
            (Box::new(mutation::mutate::Mutate::new(1)), ${MUTATE1}),
            (Box::new(mutation::mutate::Mutate::new(2)), ${MUTATE2}),
            (Box::new(mutation::mutate::Mutate::new(3)), ${MUTATE3}),
            (Box::new(mutation::rotate::Rotate::new(1)), ${ROTATE1}),
            (Box::new(mutation::rotate::Rotate::new(2)), ${ROTATE2}),
            (Box::new(mutation::rotate::Rotate::new(3)), ${ROTATE3}),
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

EOF
}

iteration () {
    local json=$(cargo run -- ${ITERATIONS})
    ELAPSED_MS=$(jq .elapsed_ms <<< ${json})
    VIOLATIONS=$(jq .violations <<< ${json})
    SCORE=$(jq .score <<< ${json})
    ITERATION=$(jq .iteration <<< ${json})
    AVERAGE_ITERATION=$(jq .average_iteration <<< ${json})

    echo ${SCORE},${VIOLATIONS},${ELAPSED_MS},${ITERATIONS},${ITERATION},${AVERAGE_ITERATION},${POPSIZE},${MUTATION_NULL},${MUTATE1},${MUTATE2},${MUTATE3},${ROTATE1},${ROTATE2},${ROTATE3},${CROSSOVER_NULL},${SPLICE},${MIX} | tee >> results.csv
}

main () {
    N=1
    CONSTANT=1
    SCALE=1.0
    EXPONENT=2.0
    CROSSOVER_NULL=2
    MIX=1
    SPLICE=1
    MUTATION_NULL=10
    MUTATE1=1
    MUTATE2=1
    MUTATE3=1
    ROTATE1=1
    ROTATE2=1
    ROTATE3=1
    POPSIZE=200
    for i in $(seq 6) ; do
        for iter in 1000 2000 5000 10000 25000 100000 ; do
            ITERATIONS=$iter
            template
            iteration
        done
    done
}

main "$@"
