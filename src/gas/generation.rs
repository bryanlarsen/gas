use super::Gas;
use crate::candidate::Candidate;

#[mockall_double::double]
use crate::rando::Rando;

#[cfg(doc)]
use crate::crossover::Crossover;
#[cfg(doc)]
use crate::fitness::FitnessConfig;
#[cfg(doc)]
use crate::mutation::Mutation;
#[cfg(doc)]
use crate::tournaments::Tournament;

/// Given one generation of candidates, create the next generation.   The heart of the GA.
///
/// Each generation consists of the following steps:
///
/// 1. Run a [Tournament] to order the candidates.
/// 2. Loop for each new child:
/// a.  Select two parents.  Parent selection is biased by [Tournament] score and prefers selecting dissimilar parents.
/// b.  Choose a [Crossover] algorithm to run on the two parents to create a child.
/// c.  Choose a [Mutation] algorithm to run on the child
///
/// These arguments could be calculated inside this function rather than
/// outside, but are taken as parameters so they don't have to be recalculated
/// every generation:
///
/// * `rng`: pass [`Rando::new()`]
/// * `score_weights`: pass [`FitnessConfig::weights()`] from [`Gas::fitness`]

impl<const N: usize, const NSYMS: usize> Gas<N, NSYMS> {
    pub fn generation(
        &self,
        population: &Vec<Candidate<N, NSYMS>>,
        rng: &mut Rando,
        score_weights: &Vec<f64>,
    ) -> Vec<Candidate<N, NSYMS>> {
        let mut nextgen = Vec::<Candidate<N, NSYMS>>::with_capacity(population.len());

        // tournament phase
        let (winner, weights) = self.cycle_tournament.run(&population, rng, score_weights);
        nextgen.push(winner);
        let mut popdist = rng.weighted_iter(&weights);

        // crossover and mutation phase
        let mut crossover_iter = self.crossovers.iter();
        let mut mutation_iter = self.mutations.iter();

        // 0 was winner of tournament, already pushed to nextgen
        for _ in 1..population.len() {
            let mut chromosone;
            loop {
                let left = &population[popdist.next().unwrap()];
                let mut right;
                let mut i = 0;
                loop {
                    right = &population[popdist.next().unwrap()];
                    if left != right {
                        if left.distance(&right) > self.taboo_distance {
                            break;
                        }
                    }
                    i += 1;
                    if i > population.len() {
                        eprintln!("TABOO!");
                        break;
                    }
                }
                let crossover = crossover_iter.next().unwrap();
                let mutation = mutation_iter.next().unwrap();
                chromosone = crossover.run(&left.chromosone, &right.chromosone, rng);
                chromosone = mutation.run(&chromosone, rng);
                if !nextgen.iter().any(|c| c.chromosone == chromosone) {
                    break;
                }
            }
            nextgen.push(Candidate::from_chromosone(&self, chromosone));
        }

        nextgen
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use mockall::predicate;

    #[test]
    fn test_generation() {
        let gas = Gas::<5, 3>::dut();
        let mut r = Rando::default();
        r.expect_shuffle().times(1).return_const(()); // used by single_elimination_tournament
        r.expect_weighted_iter() // used by generation to select parents
            .times(1)
            .return_const([0, 1].iter().cloned());
        r.expect_gen_range() // used by mutate
            .with(predicate::eq(0..5))
            .times(1)
            .return_const(1usize);
        r.expect_gen_range() // used by mutate
            .with(predicate::eq(0..3))
            .times(1)
            .return_const(1usize);

        let pop = &mut vec![
            Candidate::from_chromosone(&gas, [0, 0, 0, 0, 0]),
            Candidate::from_chromosone(&gas, [1, 0, 1, 0, 1]),
        ];
        let nextgen = gas.generation(&pop, &mut r, &vec![1.0; 9]);
        // second candidate has to win the tournament
        Candidate::assert_eq(
            &nextgen[0],
            &Candidate::from_chromosone(&gas, [1, 0, 1, 0, 1]),
        );
    }
}
