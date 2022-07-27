#[cfg(not(test))]
use crate::data::*;
#[cfg(test)]
use crate::test_data::*;

use crate::candidate::*;
use crate::config::*;
use crate::rando::*;
pub fn generation(
    population: &Vec<Candidate>,
    configuration: &Configuration,
    rng: &mut dyn Rando,
) -> Vec<Candidate> {
    assert_eq!(population.len(), POPSIZE);
    assert_eq!(
        configuration.generation.iter().fold(0, |sum, c| sum + c.n),
        POPSIZE
    );
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
        }
    }

    nextgen
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generation() {
        let mut r = RealRando::new();
        let config = configuration();
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
