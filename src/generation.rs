#[cfg(not(test))]
use crate::data::*;
#[cfg(test)]
use crate::test_data::*;

use crate::candidate::*;
use crate::config::*;

#[mockall_double::double]
use crate::rando::Rando;

#[cfg(doc)]
use crate::crossover::Crossover;
#[cfg(doc)]
use crate::mutation::Mutation;
#[cfg(doc)]
use crate::tournaments::Tournament;

/** Given one generation of candidates, create the next generation.   The heart of the GA.

Each generation consists of the following steps:

1. Run a [Tournament] to order the candidates.
2. Loop for each new child:
a.  Select two parents.  Parent selection is biased by [Tournament] score and prefers selecting dissimilar parents.
b.  Choose a [Crossover] algorithm to run on the two parents to create a child.
c.  Choose a [Mutation] algorithm to run on the child
 */

pub fn generation(
    population: &Vec<Candidate>,
    configuration: &Configuration,
    rng: &mut Rando,
) -> Vec<Candidate> {
    let mut nextgen = Vec::<Candidate>::with_capacity(population.len());

    // tournament phase
    let (winner, weights) = configuration.tournament.run(&population, rng);
    nextgen.push(winner);
    let mut popdist = rng.weighted_iter(&weights);

    // crossover and mutation phase
    let mut crossoverdist = configuration.crossover_indexes.iter().cycle();
    let mut mutationdist = configuration.mutation_indexes.iter().cycle();
    for _ in 1..population.len() {
        let mut candidate;
        loop {
            let left = &population[popdist.next().unwrap()];
            let mut right;
            loop {
                right = &population[popdist.next().unwrap()];
                if left != right {
                    if left.distance(&right) > TABOO_DISTANCE {
                        break;
                    }
                }
            }
            let crossover = &configuration.crossover[*crossoverdist.next().unwrap()].0;
            let mutation = &configuration.mutation[*mutationdist.next().unwrap()].0;
            candidate = crossover.run(&left, &right, configuration, rng);
            candidate = mutation.run(&candidate, configuration, rng);
            if !nextgen.iter().any(|c| c.chromosone == candidate.chromosone) {
                break;
            }
        }
        nextgen.push(candidate);
    }

    nextgen
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::crossover;
    use crate::fitness;
    use crate::game;
    use crate::mutation;
    use crate::tournaments;

    #[test]
    fn test_generation() {
        let mut r = Rando::default();
        r.expect_shuffle().times(1).return_const(()); // used by single_elimination_tournament
        r.expect_weighted_iter()
            .times(1)
            .return_const([0, 1].iter().cloned());
        let config = Configuration::new(InitConfiguration {
            tournament: Box::new(tournaments::single_elimination::SingleElimination::new(
                game::full::Full::new(),
            )),
            crossover: vec![(Box::new(crossover::null::Null::new()), 1)],
            mutation: vec![(Box::new(mutation::null::Null::new()), 1)],
            fitness: vec![Box::new(fitness::distance::Distance::new(7))],
            constraint: vec![],
        });

        let pop = &mut vec![
            Candidate::from_chromosone([0, 0, 0, 0, 0], &config),
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
