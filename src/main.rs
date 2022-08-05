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

mod candidate;
mod config;
mod constraints;
mod crossover;
mod fitness;
mod game;
mod generation;
mod mutation;
mod rando;
mod tournaments;

#[cfg(not(test))]
fn main() {
    use candidate::Candidate;
    use rando::Rando;

    use std::env;
    use std::time::Instant;

    let args: Vec<String> = env::args().collect();
    let mut iterations: usize = 100;
    if args.len() > 1 {
        iterations = args[1].parse().unwrap();
    }
    let mut population = Vec::<Candidate>::with_capacity(POPSIZE);
    let mut rng = Rando::new();
    let mut config = configuration();

    for _ in 0..POPSIZE {
        population.push(Candidate::new(&config, &mut rng));
    }

    let start = Instant::now();

    for i in 0..iterations {
        config.iteration = i;
        population = generation::generation(&population, &config, &mut rng);
        if i % 10 == 0 {
            eprintln!(
                "{i} {} {}",
                population[0].violations,
                population[0].total_score()
            );
        }
    }

    println!("{{\"elapsed_ms\": {},", start.elapsed().as_millis());
    print!("  \"results\":{{");
    for i in 0..LENGTH - 1 {
        print!(
            "\"{}\": {}, ",
            schedule_data::POSITION_NAMES[i],
            schedule_data::SYMBOL_NAMES[population[0].chromosone[i]]
        );
    }
    println!(
        "\"{}\": {} }},",
        schedule_data::POSITION_NAMES[LENGTH - 1],
        schedule_data::SYMBOL_NAMES[population[0].chromosone[LENGTH - 1]]
    );
    println!(
        "  \"violations\":{}, \"score\":{}, \"iteration\":{}, \"average_iteration\":{}}}",
        population[0].violations,
        population[0].total_score(),
        population[0].iteration,
        population.iter().fold(0usize, |sum, p| sum + p.iteration) as f64 / population.len() as f64,
    )
}
