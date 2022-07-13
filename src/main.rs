/*!

Each candidate is an array of LENGTH genes.   Each gene is a usize with a value in 0..NSYMS.

Each candidate results in an array of M scores.   Each score is an f64 with larger numbers being preferred.  M is expected to be a large number -- if you're aggregating scores consider using each score individually.  Each position in the score array has meaning -- the score in position i for candidate A can be compared with the score in position i for candidate B, but cannot be compared with the score in position j.

There are multiple scoring functions for each candidate.  Scoring functions each produce an array of scores which are concatenated to get the final score.

Weighting of scoring functions: TBD -- multiples in score array?

Constraints TBD.

Every generation is created from the previous generation based on the configuration.  Some individuals live on to the next generation by winning tournaments.   Others are born via mutation from a winner.   Others are born via procreation from two winners.

TBD: individuals that win more than one tournament.

 */
#[cfg(not(test))]
mod data;
#[cfg(not(test))]
mod schedule_data;
#[cfg(not(test))]
use crate::data::*;

#[cfg(test)]
mod test_data;
#[cfg(test)]
use crate::test_data::*;

mod candidate;
mod config;
mod fitness;
mod game;
mod generation;
mod rando;
mod repopulate;
mod reproduction;
mod tournaments;

use candidate::*;
#[cfg(not(test))]
use generation::*;
use rando::*;

#[cfg(test)]
use game::*;

#[cfg(test)]
use mockall::*;

#[cfg(not(test))]
use std::env;

#[cfg(not(test))]
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut iterations: usize = 100;
    if args.len() > 1 {
        iterations = args[1].parse().unwrap();
    }
    let mut population = Vec::<Candidate>::with_capacity(40);
    let mut rng = RealRando::new();
    let (config, score_config) = configuration();

    for _ in 0..POPSIZE {
        population.push(Candidate::new(&score_config, &mut rng));
    }

    for _ in 0..iterations {
        population = generation(&population, &config, &score_config, &mut rng);
        eprintln!("{}", population[0].total_score());
    }

    eprintln!(
        "{:?} {:?} {}",
        population[0].chromosone,
        population[0].scores,
        population[0].total_score()
    );

    print!("{{");
    for i in 0..LENGTH - 1 {
        print!(
            "{}: {}, ",
            schedule_data::POSITION_NAMES[i],
            schedule_data::SYMBOL_NAMES[population[0].chromosone[i]]
        );
    }
    println!(
        "{}: {} }}",
        schedule_data::POSITION_NAMES[LENGTH - 1],
        schedule_data::SYMBOL_NAMES[population[0].chromosone[LENGTH - 1]]
    );
}
