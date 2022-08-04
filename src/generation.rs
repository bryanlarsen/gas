#[cfg(not(test))]
use crate::data::*;
#[cfg(test)]
use crate::test_data::*;

use crate::candidate::*;
use crate::config::*;

#[mockall_double::double]
use crate::rando::Rando;

pub fn generation(
    population: &Vec<Candidate>,
    configuration: &Configuration,
    rng: &mut Rando,
) -> Vec<Candidate> {
    let mut nextgen = Vec::<Candidate>::with_capacity(population.len());

    for config in &configuration.generation {
        match &config.propagation {
            Propagation::Tournament(tournament) => {
                let results = tournament.run(population, rng);
                for i in (POPSIZE - config.n..POPSIZE).rev() {
                    nextgen.push(population[results[i]]);
                }
            }
            Propagation::Mutation(mutation) => {
                for _ in 0..config.n {
                    nextgen.push(mutation.run(
                        &nextgen[rng.gen_range(0..nextgen.len())],
                        configuration,
                        rng,
                    ));
                }
            }
            Propagation::Crossover(crossover) => {
                for _ in 0..config.n {
                    nextgen.push(crossover.run(
                        &nextgen[rng.gen_range(0..nextgen.len())],
                        &nextgen[rng.gen_range(0..nextgen.len())],
                        configuration,
                        rng,
                    ));
                }
            }
        }
    }

    nextgen
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::fitness;
    use crate::game;
    use crate::reproduction;
    use crate::tournaments;

    #[test]
    fn test_generation() {
        let mut r = Rando::default();
        r.expect_shuffle().times(1).return_const(());
        r.expect_gen_range().times(3).return_const(0usize);
        let config = Configuration {
            generation: vec![
                GenerationConfig {
                    n: 1,
                    propagation: Propagation::Tournament(Box::new(
                        tournaments::single_elimination::SingleElimination::new(
                            game::full::Full::new(),
                        ),
                    )),
                },
                GenerationConfig {
                    n: 1,
                    propagation: Propagation::Mutation(Box::new(
                        reproduction::mutate::Mutate::new(1),
                    )),
                },
            ],
            fitness: vec![Box::new(fitness::distance::Distance::new(7))],
            constraint: vec![],
            iteration: 0,
        };

        let pop = &mut vec![
            Candidate::from_chromosone([0, 0, 1, 0, 1], &config),
            Candidate::from_chromosone([1, 0, 1, 0, 1], &config),
        ];
        let nextgen = generation(&pop, &config, &mut r);
        // second candidate has to win the tournament
        Candidate::assert_eq(
            &nextgen[0],
            &Candidate::from_chromosone([1, 0, 1, 0, 1], &config),
        );
    }
}
