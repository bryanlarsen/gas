/*!

# Genetic Algorithm Scheduler

This is a multi-dimensional genetic algorithm based optimizer written in rust with multi-threading

This optimizer was designed for a scheduling system, and will work best for problems that have similar characteristics.   You don't have to use it for shift scheduling, it should work well for anything that meets the following characteristics:

- there are a large number of fitness functions rather than a single number or even a few numbers to be optimized.  This is the primary difference between this implementation and many others.   In our case, how well the schedule fits each preference for each employee is treated as an fitness function.   Given that the employee can specify a suitability for each shift, this means that the number of fitness functions is a multiple of the length of the chromosone.

- variables are discrete rather than continuous.   In our case, there is a small fixed number of employees that can be booked per shift.

- the "chromosone" has a fixed length.

- the position in the chromosone (aka the locus) is significant.    (In some problem domains, the locus is less significant and the ordering of the genes is more significant).

- fitness functions are relatively cheap to compute.  Parallelism is at the population rather than the game level.

Some of these characteristics are more hard coded than others.   For example, you can adjust the [FitnessFunction]'s, [Crossover] and [Mutation] functions to make the locus less significant.

## Terminology, Structure and Algorithm.

Every potential solution is called a [Candidate].   Each [Candidate] has a chromosone which defines the solution.   The chromosone is an array of unsigned integers called genes.  Each gene occupies a position in the array called the locus.   Typically genes and loci fairly directly map to real world values.

At the time of creation each [Candidate] evaluates all of its [FitnessFunction]'s to determine the [Candidate]'s suitability.  Alongside the [FitnessFunction]'s, [Constraint]'s are also evaluated and number of [Constraint] violations stored.

Because each [Candidate] has many [FitnessFunction] values, [Candidate]'s cannot be trivially and stably ordered.  Instead, two candidates are compared in a [Game], and repeated [Game]'s are run across a population in a [Tournament] to order the [Candidate]'s by [FitnessFunction]'s and [Constraint]'s.

All current [Game] implementations order by [Constraint] violations before [FitnessFunction] scores.   In other words, in other words only if two candidates have the same number of constraint violations are the fitness scores considered.  Most [Game]'s and [Tournament]'s are not stable, and have a strong stochastic component.

A set of [Candidate]'s is called a population.

Each cycle through the algorithm is called a [generation].  A generation creates a new population from an existing population.  Each generation consists of the following steps:

1. Run a [Tournament] to order the candidates.
2. Loop for each new child:
  a.  Select two parents.  Parent selection is biased by [Tournament] score and prefers selecting dissimilar parents.
  b.  Choose a [Crossover] algorithm to run on the two parents to create a child.
  c.  Choose a [Mutation] algorithm to run on the child

Looping the [generation] until stagnation is reached is called a [cycle].

Multiple [cycle]'s are run in parallel.   When a [cycle] stops TBD.

Send SIGINT (ctrl-c) to cause the algorithm to determine the current best candidate and then exit.

# Configuration

Configure the GA by creating a [Configuration] using an [InitConfiguration].   See [InitConfiguration] for documentation.

# Converting to a Library

# Tuning the GA



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
mod cycle;
mod fitness;
mod game;
mod generation;
mod mutation;
mod rando;
mod tournaments;

#[cfg(doc)]
use candidate::Candidate;
#[cfg(doc)]
use config::Configuration;
#[cfg(doc)]
use config::InitConfiguration;
#[cfg(doc)]
use constraints::Constraint;
#[cfg(doc)]
use crossover::Crossover;
#[cfg(doc)]
use fitness::FitnessFunction;
#[cfg(doc)]
use game::Game;
#[cfg(doc)]
use mutation::Mutation;
#[cfg(doc)]
use tournaments::Tournament;

#[cfg(not(test))]
fn main() {
    use candidate::Candidate;
    use rando::Rando;

    use std::sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    };
    use std::thread;
    use std::time::Instant;

    let sigint = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal_hook::consts::SIGINT, Arc::clone(&sigint)).unwrap();

    let start = Instant::now();

    const THREADS: usize = 8;

    let mut progresses = Vec::<cycle::CycleProgress>::with_capacity(THREADS);

    let mut handles = Vec::<thread::JoinHandle<Candidate>>::with_capacity(THREADS);

    for _ in 0..THREADS {
        let mut progress = cycle::CycleProgress::new(&sigint);
        progresses.push(progress.clone());

        handles.push(thread::spawn(move || {
            let mut population = Vec::<Candidate>::with_capacity(POPSIZE);
            let mut config = configuration();
            let mut rng = Rando::new();
            /*
                        for i in 0..WINNERS.len() {
                            population.push(Candidate::from_chromosone(WINNERS[i], &config));
                        }
            eprintln!("{}", Candidate::similarity(&population));
                        for _ in WINNERS.len()..POPSIZE {
                            population.push(Candidate::new(&config, &mut rng));
                        }
            eprintln!("{}", Candidate::similarity(&population));
             */
            for _ in 0..POPSIZE {
                population.push(Candidate::new(&config, &mut rng));
            }
            cycle::cycle(&mut population, &mut config, &mut progress, &mut rng)
        }));
    }

    while !sigint.load(Ordering::Relaxed) {
        thread::sleep(std::time::Duration::from_millis(1000));
        for i in 0..THREADS {
            progresses[i].eprint();
        }
        eprintln!("");
        if handles.iter().all(|h| h.is_finished()) {
            break;
        }
    }

    let mut winners: Vec<Candidate> = handles.drain(..).map(|h| h.join().unwrap()).collect();

    let progress = cycle::CycleProgress::new(&sigint);
    let mut progress1 = progress.clone();

    let handle = thread::spawn(move || {
        let mut config = configuration();
        let mut rng = Rando::new();

        cycle::cycle(&mut winners, &mut config, &mut progress1, &mut rng)
    });

    loop {
        thread::sleep(std::time::Duration::from_secs(1));
        progress.eprint();
        eprintln!("");
        if handle.is_finished() {
            break;
        }
        if sigint.load(Ordering::Relaxed) {
            break;
        }
    }

    let winner = handle.join().unwrap();

    println!("{{\"elapsed_ms\": {},", start.elapsed().as_millis());
    print!("  \"results\":{{");
    for i in 0..LENGTH - 1 {
        print!(
            "\"{}\": {}, ",
            schedule_data::POSITION_NAMES[i],
            schedule_data::SYMBOL_NAMES[winner.chromosone[i]]
        );
    }
    println!(
        "\"{}\": {} }},",
        schedule_data::POSITION_NAMES[LENGTH - 1],
        schedule_data::SYMBOL_NAMES[winner.chromosone[LENGTH - 1]]
    );
    println!(
        "  \"violations\":{}, \"score\":{}, \"iteration\":{}}}",
        winner.violations,
        winner.total_score(),
        winner.iteration,
    )
}
