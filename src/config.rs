use crate::constraints::Constraint;
use crate::crossover::Crossover;
use crate::fitness::FitnessFunction;
use crate::mutation::Mutation;
use crate::tournaments::Tournament;

/// the configuration of the GA
pub struct InitConfiguration {
    /// the tournament phase of the GA provides the weights used when selecting parents
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

fn multidimensional_bresenhams(weights: &[usize]) -> Vec<usize> {
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
