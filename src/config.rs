#[cfg(not(test))]
pub mod config {
    use crate::constraints::{self, Constraint, ConstraintConfig};
    use crate::crossover::{self, Crossover, CrossoverConfig};
    use crate::fitness::{self, FitnessConfig, FitnessFunction};
    use crate::game;
    use crate::mutation::{self, Mutation, MutationConfig};
    use crate::tournaments;

    use crate::schedule_data;

    // schedule_data.rs is automatically generated, hoist symbols

    /// the number of genes.   Genes are stored as unsigned integers from 0..NSYMS.   Mapping these to meaningful real-world values is done in [schedule_data::SYMBOL_NAMES].
    pub const NSYMS: usize = schedule_data::NSYMS;
    /// the length of a chromosone.
    pub const LENGTH: usize = schedule_data::LENGTH;

    /// number of populations to optimize simultaneously
    pub const THREADS: usize = 4;

    /// the number of [Candidate]'s in a population.
    pub const POPSIZE: usize = 200;
    /// the minimum Hamming distance between two parents.   Parents more closely related are not allowed to breed.
    pub const TABOO_DISTANCE: usize = 5;

    /// The [Tournament] and parameters to use both for ordering [Candidate]'s in a
    /// population both in a generation cycle and selecting final winners. TBD: we
    /// may want different tournaments in these two scenarios.
    ///
    /// Note that even though Tournament is a trait, because this is a constant we
    /// cannot set the type to `dyn Tournament` even though we would like to. So if
    /// you change the value of this constant you'll also have to change the type.
    pub const TOURNAMENT: tournaments::scale::Scale<
        tournaments::single_elimination::SingleElimination<game::full::Full>,
    > = tournaments::scale::Scale::new(
        tournaments::single_elimination::SingleElimination::new(game::full::Full::new()),
        1,
        1.0,
        2.0,
    );

    /// The list of [Crossover]'s that the algorithm can choose from, along with the
    /// frequency of use.
    ///
    /// Selection of the [Crossover] and [Mutation] algorithms is not done
    /// stochastically. The algorithms are cycled respecting the weights. Therefore
    /// if the sum of the weights in [CROSSOVER_CONFIG] have common factors with the
    /// sum of the weights in [MUTATION_CONFIG], some combinations of crossover and
    /// mutation may never be chosen. In some cases you may wish to deliberately
    /// trigger this effect, but in most cases you should probably ensure that the
    /// two sum of weights do not have a common factor.
    pub const CROSSOVER_CONFIG: CrossoverConfig = CrossoverConfig::new(&[
        (3, Crossover::Null),
        (2, Crossover::Splice(crossover::splice::Splice::new())),
        (2, Crossover::Mix(crossover::mix::Mix::new())),
    ]);

    /// The list of [Mutation]'s that the algorithm can choose from, along with the
    /// frequency of use.
    ///
    /// Selection of the [Crossover] and [Mutation] algorithms is not done
    /// stochastically. The algorithms are cycled respecting the weights. Therefore
    /// if the sum of the weights in [CROSSOVER_CONFIG] have common factors with the
    /// sum of the weights in [MUTATION_CONFIG], some combinations of crossover and
    /// mutation may never be chosen. In some cases you may wish to deliberately
    /// trigger this effect, but in most cases you should probably ensure that the
    /// two sum of weights do not have a common factor.
    pub const MUTATION_CONFIG: MutationConfig = MutationConfig::new(&[
        (10, Mutation::Null),
        (1, Mutation::Mutate(mutation::mutate::Mutate::new(1))),
        (1, Mutation::Mutate(mutation::mutate::Mutate::new(2))),
        (1, Mutation::Mutate(mutation::mutate::Mutate::new(3))),
        (1, Mutation::Rotate(mutation::rotate::Rotate::new(1))),
        (1, Mutation::Rotate(mutation::rotate::Rotate::new(2))),
        (1, Mutation::Rotate(mutation::rotate::Rotate::new(3))),
    ]);

    /// All of the [FitnessFunction]'s used to score a [Candidate].
    pub const FITNESS_CONFIG: FitnessConfig = FitnessConfig::new(&[
        FitnessFunction::Distance(fitness::distance::Distance::new(7)),
        FitnessFunction::ColorCount(fitness::color_count::ColorCount::new(
            schedule_data::NCOLORS,
            schedule_data::CHROMOSONE_COLORS,
            schedule_data::COLOR_PREFS,
        )),
        FitnessFunction::WeightedCount(fitness::weighted_count::WeightedCount::new(
            schedule_data::MAX_WEIGHT,
            schedule_data::WEIGHTS,
        )),
    ]);

    /// All of the [Constraint]'s applied to a candidate. Theoretically any
    /// [Candidate] that fails any constraint is invalid, but if the algorithm
    /// doesn't assume that is possible. The algorithm considers any [Candidate]
    /// with more [Constraint] violations than the minimum in the population to be
    /// inferior.
    pub const CONSTRAINT_CONFIG: ConstraintConfig =
        ConstraintConfig::new(&[Constraint::InvalidPosition(
            constraints::invalid_position::InvalidPosition::new(schedule_data::INVALID_POSITIONS),
        )]);
}

#[cfg(test)]
pub mod config {
    use crate::constraints::ConstraintConfig;
    use crate::crossover::{Crossover, CrossoverConfig};
    use crate::fitness::{self, FitnessConfig, FitnessFunction};
    use crate::game;
    use crate::mutation::{self, Mutation, MutationConfig};
    use crate::tournaments;

    pub const NSYMS: usize = 3;
    pub const LENGTH: usize = 5;
    pub const TABOO_DISTANCE: usize = 1;

    pub const CROSSOVER_CONFIG: CrossoverConfig = CrossoverConfig::new(&[(1, Crossover::Null)]);

    pub const MUTATION_CONFIG: MutationConfig =
        MutationConfig::new(&[(1, Mutation::Mutate(mutation::mutate::Mutate::new(1)))]);

    pub const FITNESS_CONFIG: FitnessConfig = FitnessConfig::new(&[FitnessFunction::Distance(
        fitness::distance::Distance::new(7),
    )]);

    pub const CONSTRAINT_CONFIG: ConstraintConfig = ConstraintConfig::new(&[]);

    pub const TOURNAMENT: tournaments::single_elimination::SingleElimination<game::full::Full> =
        tournaments::single_elimination::SingleElimination::new(game::full::Full::new());
}
