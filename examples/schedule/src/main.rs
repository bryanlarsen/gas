mod schedule_data;

use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread;
use std::time::Instant;

use gas::chromosone;
use gas::constraints::{self, ConstraintConfig};
use gas::crossover::{self, CrossoverConfig};
use gas::fitness::{self, FitnessConfig};
use gas::game;
use gas::gas::Gas;
use gas::mutation::{self, MutationConfig};
use gas::pool::Pool;
use gas::tournaments;

const NTHREADS: usize = 4;

fn main() {
    let gas = Arc::new(Gas {
        fitness: FitnessConfig::new(vec![
            Box::new(fitness::distance::Distance::new(
                7,
                schedule_data::DISTANCE_BEFORE,
                [usize::MAX; chromosone::NSYMS],
                1.0,
                1.0,
            )),
            Box::new(fitness::color_count::ColorCount::new(
                schedule_data::NCOLORS,
                schedule_data::CHROMOSONE_COLORS.to_vec(),
                schedule_data::COLOR_PREFS
                    .iter()
                    .map(|a| a.to_vec())
                    .collect(),
                &schedule_data::COLOR_NAMES,
                1.0,
            )),
            Box::new(fitness::weighted_count::WeightedCount::new(
                schedule_data::MAX_WEIGHT,
                schedule_data::WEIGHTS.iter().map(|v| v.to_vec()).collect(),
            )),
        ]),
        constraints: ConstraintConfig::new(vec![Box::new(
            constraints::invalid_position::InvalidPosition::new(
                schedule_data::INVALID_POSITIONS
                    .iter()
                    .map(|v| v.to_vec())
                    .collect(),
            ),
        )]),
        cycle_tournament: Box::new(tournaments::scale::Scale::new(
            tournaments::single_elimination::SingleElimination::new(game::full::Full::new()),
            1,
            1.0,
            2.0,
        )),
        final_tournament: Box::new(tournaments::full_season::FullSeason::new(
            game::full::Full::new(),
        )),
        crossovers: CrossoverConfig::new(vec![
            (3, Box::new(crossover::null::Null::new())),
            (2, Box::new(crossover::splice::Splice::new())),
            (2, Box::new(crossover::mix::Mix::new())),
        ]),
        mutations: MutationConfig::new(vec![
            (10, Box::new(mutation::null::Null::new())),
            (1, Box::new(mutation::mutate::Mutate::new(1))),
            (1, Box::new(mutation::mutate::Mutate::new(2))),
            (1, Box::new(mutation::mutate::Mutate::new(3))),
            (1, Box::new(mutation::rotate::Rotate::new(1))),
            (1, Box::new(mutation::rotate::Rotate::new(2))),
            (1, Box::new(mutation::rotate::Rotate::new(3))),
        ]),
        taboo_distance: 5,
        population_size: 200,
    });

    let sigint = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&sigint)).unwrap();
    let start = Instant::now();

    let mut pool = Pool::new(gas.clone(), NTHREADS, sigint);

    while !pool.is_finished() {
        for i in 0..NTHREADS {
            pool.progresses[i].eprint();
            //eprintln!("{:?}", pool.progresses[i].top.read().unwrap().chromosone);
        }
        eprintln!("");
        thread::sleep(std::time::Duration::from_millis(1000));
    }

    let winner = pool.winner(gas.clone());

    println!("{{\"elapsed_ms\": {},", start.elapsed().as_millis());
    println!("  \"chromosone\":{:?},", winner.chromosone);
    print!("  \"results\":{{");
    for i in 0..chromosone::LENGTH - 1 {
        print!(
            "\"{}\": \"{}\", ",
            schedule_data::LOCUS_NAMES[i],
            schedule_data::SYMBOL_NAMES[winner.chromosone[i] as usize]
        );
    }
    println!(
        "\"{}\": \"{}\" }},",
        schedule_data::LOCUS_NAMES[chromosone::LENGTH - 1],
        schedule_data::SYMBOL_NAMES[winner.chromosone[chromosone::LENGTH - 1] as usize]
    );
    println!(
        "  \"violations\":{}, \"score\":{}}}",
        winner.violations,
        winner.total_score(&gas.fitness.weights()),
    )
}
