/*!

# Genetic Algorithm Scheduler

This is a multi-dimensional genetic algorithm based optimizer written in rust with multi-threading.

## Distinguishing Features

- supports highly dimensional fitness scores with multiple mechanisms to order candidates

- does not require a stable ordering between candidates

- supports both constraint violations and fitness scores

- takes advantage of const generics

This optimizer was designed for a scheduling system, and will work best for problems that have similar characteristics.   You don't have to use it for shift scheduling, it should work well for anything that meets the following characteristics:

- there are a large number of fitness functions rather than a single number or even a few numbers to be optimized.  In our case, how well the schedule fits each preference for each employee is treated as an fitness function.   Given that the employee can specify a suitability for each shift, this means that the number of fitness scores is larger than the length of the chromosone.

- variables are discrete rather than continuous.   In our case, there are a small fixed number of employees that can be booked per shift.

- the "chromosone" has a fixed length.

- the position in the chromosone (aka the locus) is significant.    (In some other problem domains, the locus is less significant and the ordering of the genes is more significant).  This limitation can be overcome by writing significantly different [FitnessFunction]'s.

- fitness functions are relatively cheap to compute.  Parallelism is at the population rather than the game level.

Some of these characteristics are more hard coded than others.   For example, you can adjust the [FitnessFunction]'s, [Crossover] and [Mutation] functions to make genome subsequences more significant than locus position.

## Terminology, Structure and Algorithm.

Every potential solution is called a [Candidate].   Each [Candidate] has a chromosone which defines the solution.   The chromosone is an array of unsigned integers called genes.  Each gene occupies a position in the array called the locus.   For ease of computation, genes and locii are not enums, but adjacent unsized integers.   Mapping these integers to values is outside the scope of this library.   See the examples for details.

At the time of creation each [Candidate] evaluates all of its [FitnessFunction]'s to determine the [Candidate]'s suitability.  Alongside the [FitnessFunction]'s, [Constraint]'s are also evaluated and number of [Constraint] violations stored.

Because each [Candidate] has many [FitnessFunction] values, [Candidate]'s cannot be trivially and stably ordered.  Instead, two candidates are compared in a [Game], and repeated [Game]'s are run across a population in a [Tournament] to order the [Candidate]'s by [FitnessFunction]'s and [Constraint]'s.

All current [Game] implementations order by [Constraint] violations before [FitnessFunction] scores.   In other words, only if two candidates have the same number of constraint violations are the fitness scores considered.  Most [Game]'s and [Tournament]'s are not stable, and have a strong stochastic component.

A set of [Candidate]'s is called a population.

Each cycle through the algorithm is called a [Gas::generation].  A generation creates a new population from an existing population.  Each generation consists of the following steps:

1. Run a [Tournament] to order the candidates.
2. Loop for each new child:
    1.  Select two parents.  Parent selection is biased by [Tournament] score and prefers selecting dissimilar parents.
    2.  Choose a [Crossover] algorithm to run on the two parents to create a child.
    3.  Choose a [Mutation] algorithm to run on the child

Typically the GA is tuned so that Null [Crossover] and [Mutation] algorithms are
often chosen. The Null algorithms simply clone a parent rather than performing a
crossover or mutation.

Selection of the [Crossover] and [Mutation] algorithms are not done stochastically.  Weights are configured for each algorithm, and the algorithms are cycled respecting the weights.   Therefore if the sum of the weights in [Gas::crossovers] have common factors with the sum of the weights in [Gas::mutations], some combinations of crossover and mutation may never be chosen.   In some cases you may wish to deliberately trigger this effect, but in most cases you should probably ensure that the both sum of weights do not have a common factor.

Looping the [Gas::generation] until stagnation is reached is called a [Gas::cycle].   There are three stages to each cycle: seeding, running and finalizing.   See the docs for [Gas::cycle] for more details

Multiple [Gas::cycle]'s are run in parallel in a [pool]. The pool may be terminated
early by setting the [CycleProgress::sigint] flag. When a [Gas::cycle] stops TBD.

# Setup and Configuration

There is a simple full example in examples/schedule and a minimal configuration in the `#[cfg(test)] Gas::dut()`.

## Example

Let's play Mastermind!   We've got 6 colors and 4 positions, so the chromosone type parameters are `<N=4, NSYMS=6>`.

If we were really playing the game, our two fitness functions would be the number of white pegs and number of black pegs.   However, we'll use this example to show why it is advantageous to have a lot of fitness functions.

So instead of having the number of black pegs as the fitness function, we'll return 4 separate fitness functions for each possible black peg, and 6 separate fitness functions for each possible white peg.

Here's our fitness function for the black pegs.   We'll use the delta between the desired and actual as the score.   For delta, 0 is the best score, but GAS optimizes for highest score.   So for the score here we return the negated delta -- zero is the highest negative number.

```
#
# use gas::chromosone::Gene;
#
# use gas::fitness::{FitnessFunction, FitnessName};
#
pub struct Black<const N: usize, const NSYMS: usize> {
  pub answer: [Gene; N]
}

impl<const N: usize, const NSYMS: usize> FitnessFunction<N, NSYMS> for Black<N, NSYMS> {
  fn nscores(&self) -> usize { N }

  fn run(&self, chromosone: &[Gene; N]) -> Vec<f64> {
    std::iter::zip(chromosone, self.answer).map(|(guess, desired)|
      -(*guess as f64 - desired as f64).abs()
    ).collect()
  }
}

assert_eq!(Black::<4,6>{answer:[4,3,2,1]}.run(&[4,0,2,0]), vec![0.0, -3.0, 0.0, -1.0]);
```

For the white scores, we could similarly create a [FitnessFunction] that returns 6 scores, counting each gene and returning how close each count is to the count in the answer.   But there is already a FitnessFunction that does this in the library: [fitness::ColorCount].

Once you have the chromosone and the [`FitnessFunction`] defined, you can configure the optimizer by creating a [Gas] object.

```
#
# use gas::chromosone::Gene;
#
# use gas::fitness::{FitnessFunction, FitnessName};
#
# pub struct Black<const N: usize, const NSYMS: usize> {
# pub answer: [Gene; N]
# }
#
# impl<const N: usize, const NSYMS: usize> FitnessFunction<N, NSYMS> for Black<N, NSYMS> {
#  fn nscores(&self) -> usize { N }
#
#  fn run(&self, chromosone: &[Gene; N]) -> Vec<f64> {
#    std::iter::zip(chromosone, self.answer).map(|(guess, desired)|
#      -(*guess as f64 - desired as f64).abs()
#    ).collect()
#  }
# }
#
# use gas::Gas;
# use gas::fitness::{FitnessConfig, self};
# use gas::constraints::{ConstraintConfig, self};
# use gas::game;
# use gas::mutation::{MutationConfig, self};
# use gas::tournaments;
# use gas::crossover::{self, CrossoverConfig};

let gas = Gas {
  fitness: FitnessConfig::new(vec![
    Box::new(Black::<4, 6>{answer: [4,3,2,1]}),
    Box::new(fitness::ColorCount::<4,6>::new(1, vec![0], vec![vec![0],vec![1],vec![1],vec![1],vec![1],vec![0]], &[""], 1.0)),
  ]),
  constraints: ConstraintConfig::new(vec![]),
  cycle_tournament: Box::new(tournaments::SingleElimination::new(game::Full::new())),
  final_tournament: Box::new(tournaments::FullSeason::new(game::Full::new())),
  crossovers: CrossoverConfig::new(vec![(1, Box::new(crossover::Null::new()))]),
  mutations: MutationConfig::new(vec![
    (1, Box::new(mutation::Null::new())),
    (1, Box::new(mutation::Mutate::<4, 6>::new(1))),
    (1, Box::new(mutation::Rotate::<4, 6>::new(1))),
  ]),
  taboo_distance: 1,
  population_size: 10,
};
```

## The Chromosone

The chromosone is configured through const generics.   Virtually every object in the system takes two const generics as template parameters: `N` and `NSYMS`.   `N` is the length of the chromosone, and `NSYMS` is the number of genetic symbols there are.   The chromosone is defined as `[Gene; N]` where Gene is in the range `0..NSYMS`.  In chromosone.rs Gene is defined as a u8 since there are very few genetic algorithm problems where NSYMS is greater than 256.   Theoretically the `pub type Gene = u8` could be templated as well, if you need it done, feel free to ask or open a PR.

## The `Gas` object

The algorithm is configured by creating a [Gas] object.   This object specifies the fitness functions, constraints, mutation and crossover algorithms, et cetera.

### Fitness Functions

It's quite likely that the available fitness functions will not be suitable for your project.   A good set of fitness functions are critical to the success of your optimization.  It's easy to spend lots of effort on algorithm tuning, but without a solid base you're unlikely to get good results.

[`FitnessFunction`]'s return a Vec of floats for the scores, taking a chromosone as input.

### Constraints

[`Constraint`]'s are much like fitness functions, except they return vectors of booleans, specifying whether the chromosone is valid or invalid.  Any candidates which have more constraint violations than others will lose any competitions.  A boolean doesn't provide much guidance to the optimizer, so providing a fitness function that will indicate if a chromosone is close to violating a constraint will be helpful.

### Crossovers and Mutations

The set of [`Crossover`] and [`Mutation`] operators provided in the example are probably a good starting point for your problem.   One important consideration is to provide a good number of Null operators for both.

### Games and Tournaments

A [`Game`] replaces the simple fitness score competition in most Genetic Algorithms, so it is an interesting area for experiment.  The one used in the example worked best for us.   A [`Tournament`] is used to rank candidates.

### Taboo Distance

This ensures genetic diversity

### Stagnation parameters

There are some constants in the [`Gas::cycle`] function that are used to determine when a cycle of the algorithm has stagnated.   They perhaps should be parameterized, but for now can

### nthreads and the Pool

Once you have a [`Gas`] configured, you can start it via [`Gas::cycle`], or instead you can set up [`Pool`] to run several populations in parallel.  [Gas::cycle] takes a [CycleProgress] as a parameter -- you can monitor the progress of the optimizer by monitoring the [CycleProgress] from a separate thread.

```
#
# use gas::chromosone::Gene;
#
# use gas::fitness::{FitnessFunction, FitnessName};
#
# pub struct Black<const N: usize, const NSYMS: usize> {
# pub answer: [Gene; N]
# }
#
# impl<const N: usize, const NSYMS: usize> FitnessFunction<N, NSYMS> for Black<N, NSYMS> {
#  fn nscores(&self) -> usize { N }
#
#  fn run(&self, chromosone: &[Gene; N]) -> Vec<f64> {
#    std::iter::zip(chromosone, self.answer).map(|(guess, desired)|
#      -(*guess as f64 - desired as f64).abs()
#    ).collect()
#  }
# }
#
# use gas::Gas;
# use gas::fitness::{FitnessConfig, self};
# use gas::constraints::{ConstraintConfig, self};
# use gas::game;
# use gas::mutation::{MutationConfig, self};
# use gas::tournaments;
# use gas::crossover::{self, CrossoverConfig};
#
# let gas = Gas {
#  fitness: FitnessConfig::new(vec![
#    Box::new(Black::<4, 6>{answer: [4,3,2,1]}),
#    Box::new(fitness::ColorCount::<4,6>::new(1, vec![0; 4], vec![vec![0],vec![1],vec![1],vec![1],vec![1],vec![0]], &[""], 1.0)),
#  ]),
#  constraints: ConstraintConfig::new(vec![]),
#  cycle_tournament: Box::new(tournaments::SingleElimination::new(game::Full::new())),
#  final_tournament: Box::new(tournaments::FullSeason::new(game::Full::new())),
#  crossovers: CrossoverConfig::new(vec![(1, Box::new(crossover::Null::new()))]),
#  mutations: MutationConfig::new(vec![
#    (1, Box::new(mutation::Null::new())),
#    (1, Box::new(mutation::Mutate::<4, 6>::new(1))),
#    (1, Box::new(mutation::Rotate::<4, 6>::new(1))),
#  ]),
#  taboo_distance: 1,
#  population_size: 10,
# };
# use std::sync::atomic::AtomicBool;
# use std::sync::Arc;
# use gas::gas::cycle::CycleProgress;

let sigint = Arc::new(AtomicBool::new(false));
let mut progress = CycleProgress::<4, 6>::new(&gas, &sigint);
let solution = gas.cycle(&mut progress);
assert_eq!(solution.chromosone, [4,3,2,1]);
```



# References

<https://www.researchgate.net/profile/Marek-Gutowski/publication/216486175_Biology_Physics_Small_Worlds_and_Genetic_Algorithms/links/09e415003ec1521171000000/Biology-Physics-Small-Worlds-and-Genetic-Algorithms.pdf?origin=publication_detail>

<https://www.ncbi.nlm.nih.gov/pmc/articles/PMC7259647/>

# Future

## unwrap

This library makes extensive use of unwrap.  This can be justified by the fact that the data given to it is programmatic and the configuration set at compile time, so theoretically pretty much any error is programmatic rather than usage.   But really it's just because unwrap style programming is easier.   I do plan on eventually putting in proper error handling, but there are always higher priorities.  I deliberately used unwrap to make searching and replacing them easier.   Although technically there are a lot of vector and array accesses using `[]` that would be safer using `.get()`.

## further development

This library was designed to be used in a multi-population network.  Some effort was made to implement a primitive network topology between the populations, but this did not improve the results and was abandoned.  This should be explored in more depth.   See <https://www.ncbi.nlm.nih.gov/pmc/articles/PMC7259647/> for a discussion of potential network topologies.


 */

pub mod candidate;
pub mod chromosone;
pub mod constraints;
pub mod crossover;
pub mod fitness;
pub mod game;
pub mod gas;
pub mod helpers;
pub mod mutation;
pub mod pool;
pub mod rando;
pub mod tournaments;

pub use crate::gas::Gas;

#[cfg(doc)]
use crate::gas::cycle::CycleProgress;
#[cfg(doc)]
use candidate::Candidate;
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
use pool::Pool;
#[cfg(doc)]
use tournaments::Tournament;
