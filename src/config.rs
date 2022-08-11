use crate::constraints::Constraint;
use crate::crossover::Crossover;
use crate::fitness::FitnessFunction;
use crate::mutation::Mutation;
use crate::tournaments::Tournament;

#[cfg(doc)]
use crate::candidate::Candidate;

/**
 * To configure the GA, pass one of these structures to [Configuration::new].
 * [InitConfiguration.fitness] and [InitConfiguration.constraint] are lists of
 * the [FitnessFunction]'s and [Constraint]'s used to score an individual
 * [Candidate]. [InitConfiguration.tournament] is used to rank a population of
 * candidates. [InitConfiguration.crossover] and [InitConfiguration.mutation]
 * are weighted lists which specify which algorithms are used during those
 * phases of the algorithm. Each child has a single crossover and a single
 * mutation algorithm applied to it. There are null variants of the crossover
 * and mutation traits available so that these steps may be sometimes skipped.
 *
 * The two weighted lists use each element (weight / sum of weights) percent of
 * the time. This is deterministic, not stochastic -- if there are two elements
 * with equal weights candidates will alternate between them.
 * [multidimensional_bresenhams] is used to distribute the weights.
*
* Example:
```
let config = Configuration::new(InitConfiguration {
  tournament: Box::new(tournaments::single_elimination::SingleElimination::new(
    game::sample::Sample::new(TRIES_PER_GAME),
  )),
  crossover: vec![],
  mutation: vec![
    (Box::new(mutation::null::Null::new()), 9)
    (Box::new(mutation::mutate::Mutate::new(1)), 1)
  ],
  fitness: vec![Box::new(fitness::distance::Distance::new(7))],
  constraint: vec![],
})
```
*
 */

pub struct InitConfiguration {
    /// the [Tournament] phase of the GA provides the weights used when selecting parents
    pub tournament: Box<dyn Tournament>,
    /// which crossover algorithms to use, along with weights indicating ratio of use
    pub crossover: Vec<(Box<dyn Crossover>, usize)>,
    /// which mutation algorithms to use, along with weights.
    pub mutation: Vec<(Box<dyn Mutation>, usize)>,
    /// the fitness functions that score a candidate
    pub fitness: Vec<Box<dyn FitnessFunction>>,
    /// constraints are more important than fitness functions
    pub constraint: Vec<Box<dyn Constraint>>,
}

/// the configuration.  Basically [InitConfiguration] in working format
pub struct Configuration {
    /// see InitConfig
    pub tournament: Box<dyn Tournament>,
    /// see InitConfig
    pub crossover: Vec<(Box<dyn Crossover>, usize)>,
    /// see InitConfig
    pub mutation: Vec<(Box<dyn Mutation>, usize)>,
    /// see InitConfig
    pub fitness: Vec<Box<dyn FitnessFunction>>,
    /// see InitConfig
    pub constraint: Vec<Box<dyn Constraint>>,
    /// generation number, aka iteration number
    pub iteration: usize,
    /// if you use these indexes to step through the crossover vec, you will sample crossover appropriately
    pub crossover_indexes: Vec<usize>,
    /// if you use these indexes to step through the mutation vec, you will sample mutation appropriately
    pub mutation_indexes: Vec<usize>,
}

pub struct MutationIter<'a> {
    pub config: &'a Configuration,
    pub i: usize,
}

impl<'a> Iterator for MutationIter<'a> {
    type Item = &'a Box<dyn Mutation>;

    fn next(&mut self) -> Option<&'a Box<dyn Mutation>> {
        self.i += 1;
        if self.i >= self.config.mutation.len() {
            self.i = 0;
        }
        Some(&self.config.mutation[self.i].0)
    }
}

pub fn multidimensional_bresenhams(weights: &[usize]) -> Vec<usize> {
    let mut result: Vec<usize> = vec![];
    if weights.len() == 0 {
        return result;
    }
    let max = *weights.iter().max().unwrap() as isize;
    let mut errs: Vec<isize> = vec![0; weights.len()];
    for _ in 0..max {
        for i in 0..weights.len() {
            if errs[i] >= 0 {
                result.push(i);
                errs[i] -= max;
            }
            errs[i] += weights[i] as isize;
        }
    }
    result
}

impl Configuration {
    pub fn new(init_config: InitConfiguration) -> Configuration {
        Configuration {
            iteration: 0,
            crossover_indexes: multidimensional_bresenhams(
                &init_config
                    .crossover
                    .iter()
                    .map(|c| c.1)
                    .collect::<Vec<usize>>(),
            ),
            mutation_indexes: multidimensional_bresenhams(
                &init_config
                    .mutation
                    .iter()
                    .map(|c| c.1)
                    .collect::<Vec<usize>>(),
            ),
            tournament: init_config.tournament,
            crossover: init_config.crossover,
            mutation: init_config.mutation,
            fitness: init_config.fitness,
            constraint: init_config.constraint,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::multidimensional_bresenhams;

    #[test]
    fn test_bres() {
        assert_eq!(
            &multidimensional_bresenhams(&[2usize, 3, 1]),
            &[0, 1, 2, 1, 0, 1]
        );
    }
}
