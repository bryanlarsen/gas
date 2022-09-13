# Genetic Algorithm Scheduler

(This README is extracted from the [documentation](https://bryanlarsen.github.io/gas/gas/index.html), which is more authoritative and extensive).

This is a multi-dimensional genetic algorithm based optimizer written in rust with multi-threading

This optimizer was designed for a scheduling system, and will work best for problems that have similar characteristics.   You don't have to use it for shift scheduling, it should work well for anything that meets the following characteristics:

- there are a large number of fitness functions rather than a single number or even a few numbers to be optimized.  This is the primary difference between this implementation and many others.   In our case, how well the schedule fits each preference for each employee is treated as an fitness function.   Given that the employee can specify a suitability for each shift, this means that the number of fitness functions is a multiple of the length of the chromosone.

- variables are discrete rather than continuous.   In our case, there are a small fixed number of employees that can be booked per shift.

- the "chromosone" has a fixed length.

- the position in the chromosone (aka the locus) is significant.    (In some other problem domains, the locus is less significant and the ordering of the genes is more significant).

- fitness functions are relatively cheap to compute.  Parallelism is at the population rather than the game level.

Some of these characteristics are more hard coded than others.   For example, you can adjust the [FitnessFunction]'s, [Crossover] and [Mutation] functions to make the genome relationships more significant.

