use crate::config::config::{
    CROSSOVER_CONFIG, LENGTH, MUTATION_CONFIG, NSYMS, TABOO_DISTANCE, TOURNAMENT,
};

use crate::candidate::Candidate;
use crate::tournaments::Tournament;

#[mockall_double::double]
use crate::rando::Rando;

#[cfg(doc)]
use crate::crossover::Crossover;
#[cfg(doc)]
use crate::mutation::Mutation;

/** Given one generation of candidates, create the next generation.   The heart of the GA.

Each generation consists of the following steps:

1. Run a [Tournament] to order the candidates.
2. Loop for each new child:
a.  Select two parents.  Parent selection is biased by [Tournament] score and prefers selecting dissimilar parents.
b.  Choose a [Crossover] algorithm to run on the two parents to create a child.
c.  Choose a [Mutation] algorithm to run on the child
 */

pub fn generation(population: &Vec<Candidate>, rng: &mut Rando) -> Vec<Candidate> {
    let mut nextgen = Vec::<Candidate>::with_capacity(population.len());

    // tournament phase
    let (winner, weights) = TOURNAMENT.run(&population, rng);
    nextgen.push(winner);
    let mut popdist = rng.weighted_iter(&weights);

    // crossover and mutation phase
    let mut crossover_iter = CROSSOVER_CONFIG.iter();
    let mut mutation_iter = MUTATION_CONFIG.iter();
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
            let crossover = crossover_iter.next().unwrap();
            let mutation = mutation_iter.next().unwrap();
            candidate = crossover.run(&left, &right, rng);
            candidate = mutation.run(&candidate, rng);
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

    use mockall::predicate;

    #[test]
    fn test_generation() {
        let mut r = Rando::default();
        r.expect_shuffle().times(1).return_const(()); // used by single_elimination_tournament
        r.expect_weighted_iter() // used by generation to select parents
            .times(1)
            .return_const([0, 1].iter().cloned());
        r.expect_gen_range() // used by mutate
            .with(predicate::eq(0..LENGTH))
            .times(1)
            .return_const(1usize);
        r.expect_gen_range() // used by mutate
            .with(predicate::eq(0..NSYMS))
            .times(1)
            .return_const(1usize);

        let pop = &mut vec![
            Candidate::from_chromosone([0, 0, 0, 0, 0]),
            Candidate::from_chromosone([1, 0, 1, 0, 1]),
        ];
        let nextgen = generation(&pop, &mut r);
        // second candidate has to win the tournament
        Candidate::assert_eq(&nextgen[0], &Candidate::from_chromosone([1, 0, 1, 0, 1]));
    }
}
