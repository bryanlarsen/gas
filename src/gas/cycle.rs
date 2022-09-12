use super::Gas;
use crate::candidate::Candidate;
#[mockall_double::double]
use crate::rando::Rando;

use std::sync::{
    atomic::{AtomicBool, AtomicIsize, AtomicUsize, Ordering},
    Arc, RwLock,
};

#[cfg_attr(test, allow(dead_code))]
/** Communication between a [Gas.cycle] running in a thread and the main thread.   This allows the GA algorithm to be monitored during execution.  */
pub struct CycleProgress<const N: usize, const NSYMS: usize> {
    /// out: the number of iterations of the GA that have been run
    pub iteration: Arc<AtomicUsize>,
    /// out: continually updated with a rounded integer of [Candidate#total_score] of the best candidate
    pub score: Arc<AtomicIsize>,
    /// out: continually updated with the number of constraint violations of the best candidate
    pub violations: Arc<AtomicUsize>,
    /// out: continually updated with progress, values 0-100.   Does not increment until stagnation is detected.
    pub progress: Arc<AtomicUsize>,
    /// out: copy of the top candidate
    pub top: Arc<RwLock<Candidate<N, NSYMS>>>,
    /// in: SIGINT or similar.  if set, cycle will finish and exit ASAP
    pub sigint: Arc<AtomicBool>,

    pub seed_pool_size: Arc<AtomicUsize>,
    pub diversity_violations: Arc<AtomicUsize>,
}

#[cfg_attr(test, allow(dead_code))]
impl<const N: usize, const NSYMS: usize> CycleProgress<N, NSYMS> {
    pub fn new(gas: &Gas<N, NSYMS>, sigint: &Arc<AtomicBool>) -> CycleProgress<N, NSYMS> {
        CycleProgress {
            iteration: Arc::new(AtomicUsize::new(0)),
            score: Arc::new(AtomicIsize::new(0)),
            violations: Arc::new(AtomicUsize::new(0)),
            progress: Arc::new(AtomicUsize::new(0)),
            seed_pool_size: Arc::new(AtomicUsize::new(0)),
            diversity_violations: Arc::new(AtomicUsize::new(0)),
            top: Arc::new(RwLock::new(Candidate::from_chromosone(gas, [0; N]))),
            sigint: Arc::clone(&sigint),
        }
    }

    /// Arc::clone all the Arc's.   So like Arc::clone, doesn't actually clone the contents of the CycleProgress, just the wrapper.
    pub fn clone(&self) -> CycleProgress<N, NSYMS> {
        CycleProgress {
            iteration: Arc::clone(&self.iteration),
            score: Arc::clone(&self.score),
            violations: Arc::clone(&self.violations),
            progress: Arc::clone(&self.progress),
            seed_pool_size: Arc::clone(&self.seed_pool_size),
            diversity_violations: Arc::clone(&self.diversity_violations),
            top: Arc::clone(&self.top),
            sigint: Arc::clone(&self.sigint),
        }
    }

    pub fn eprint(&self) {
        eprint!(
            "[{}: {}/{}/{}-{} {}%]\t",
            self.iteration.load(Ordering::Relaxed),
            self.score.load(Ordering::Relaxed),
            self.violations.load(Ordering::Relaxed),
            self.seed_pool_size.load(Ordering::Relaxed),
            self.diversity_violations.load(Ordering::Relaxed),
            self.progress.load(Ordering::Relaxed),
        );
    }
}

/**
 ** Given a population, run multiple [Gas.generation]'s of the algorithm until it stagnates and then return the winner.
 **
 ** This function is designed to be run in a thread, it keeps the [CycleProgress]
 ** structure updated which allows the function to be monitored on the fly.
 **
 ** There are four phases to the algorithm.
 **
 ** 1. Seeding: Starting with a set of random candidates, iterate [Gas.generation] until the top
 ** candidate either has zero constraint violations or the number of constraint
 ** violations has stabilized. Add the candidate to the seed and then restart
 ** with another set of random candidates. Repeat until you've accumulated
 ** population_size number of seeds.
 **
 **  2. Running: Iterate [Gas.generation] until both the score and number of violations has stagnated.
 **
 ** 3. Stagnated: Keep running the GA, doing a biased sampling of the winners
 ** until we've got enough samples or we've been doing this step for too long.
 **
 ** 4. Do a final tournament of the winners do get the grand winner, which we return.
 **
 **/
impl<const N: usize, const NSYMS: usize> Gas<N, NSYMS> {
    #[cfg_attr(test, allow(dead_code))]
    pub fn cycle(&self, progress: &mut CycleProgress<N, NSYMS>) -> Candidate<N, NSYMS> {
        let score_weights = self.fitness.weights();
        let mut population = Vec::<Candidate<N, NSYMS>>::with_capacity(self.population_size);
        let mut rng = Rando::new();
        let mut seed_pool = Vec::<Candidate<N, NSYMS>>::new();

        for _ in 0..self.population_size {
            population.push(Candidate::new(self, &mut rng));
        }

        // fast moving average of score.  Stagnation of score is defined as when this goes below the slow moving average.
        let mut ema99 = population[0].total_score(&score_weights);
        // slow moving average of score
        let mut ema999 = ema99;
        // number of violations of current winner
        let mut cur_violations = population[0].violations;
        // how long the number of violations has stayed stable.  Stagnation is defined as when this reaches VIOLATIONS_STAGNATION_THRESHOLD
        let mut n_cur_violations = 1;
        // best score ever seen
        let mut best_score = 0f64;
        // a sampling of generation winners for the final tournament
        let mut winners = Vec::<Candidate<N, NSYMS>>::with_capacity(population.len());
        // when state flipped
        let mut stagnation_iteration = 1usize;

        // Stagnation tuning parameters.
        /// the exponential smoothing constant for the fast moving average
        const EMA_FAST_CONST: f64 = 0.995;
        /// the exponential smoothing constant for the slow moving average
        const EMA_SLOW_CONST: f64 = 0.9995;
        /// how long the number of violations can stay at a single value before it is considered stagnate.   Hopefully the value at this point is 0!
        const VIOLATIONS_STAGNATION_THRESHOLD: usize = 100;
        /// usually we sample winners until we have population.len() samples, but we also stop if we've spent too long, defined here.  Is a multiple of the number of iterations before stagnation.
        const SAMPLING_LENGTH: usize = 3;

        enum State {
            Seeding,
            Running,
            Stagnated,
        }
        let mut state = State::Seeding;

        // seed so on sigint it's not empty
        winners.push(population[0].clone());
        seed_pool.push(population[0].clone());

        for i in 0..(2 << 20) {
            progress.iteration.store(i, Ordering::Relaxed);

            population = self.generation(&population, &mut rng, &score_weights);

            let ts = population[0].total_score(&score_weights);
            progress.score.store(ts.round() as isize, Ordering::Relaxed);
            progress
                .violations
                .store(population[0].violations, Ordering::Relaxed);

            match progress.top.try_write() {
                Err(_) => (),
                Ok(mut l) => *l = population[0].clone(),
            }

            match state {
                State::Seeding => {
                    if population[0].violations == 0
                        || (n_cur_violations > VIOLATIONS_STAGNATION_THRESHOLD
                            && population[0].violations <= seed_pool[0].violations)
                    {
                        if population[0].violations < seed_pool[0].violations {
                            seed_pool.clear();
                        }
                        seed_pool.push(population[0].clone());
                        if seed_pool.len() == self.population_size {
                            population = seed_pool.clone();
                            state = State::Running;
                        } else {
                            population.clear();
                            for _ in 0..self.population_size {
                                population.push(Candidate::new(self, &mut rng));
                            }
                            cur_violations = population[0].violations;
                            n_cur_violations = 0;
                        }
                    } else {
                        if population[0].violations < cur_violations {
                            cur_violations = population[0].violations;
                            n_cur_violations = 1;
                        } else if population[0].violations == cur_violations {
                            n_cur_violations += 1;
                        }
                    }
                    progress
                        .seed_pool_size
                        .store(seed_pool.len(), Ordering::Relaxed);
                    progress
                        .diversity_violations
                        .store(seed_pool[0].violations, Ordering::Relaxed);
                }
                State::Running => {
                    if ts > best_score {
                        best_score = ts;
                        winners[0] = population[0].clone();
                    }

                    ema99 = ema99 * EMA_FAST_CONST + ts * (1.0 - EMA_FAST_CONST);
                    ema999 = ema999 * EMA_SLOW_CONST + ts * (1.0 - EMA_SLOW_CONST);

                    if cur_violations == population[0].violations {
                        n_cur_violations += 1;
                    } else {
                        cur_violations = population[0].violations;
                        n_cur_violations = 1;
                    }
                    if ema99 < ema999 && n_cur_violations > VIOLATIONS_STAGNATION_THRESHOLD {
                        state = State::Stagnated;
                        stagnation_iteration = i;
                        winners.push(population[0].clone());
                    }
                }
                State::Stagnated => {
                    if ts > best_score {
                        if !winners
                            .iter()
                            .any(|c| c.chromosone == population[0].chromosone)
                        {
                            winners.push(population[0].clone());
                            if winners.len() >= population.len() {
                                break;
                            }
                        }
                    }
                    if i > (SAMPLING_LENGTH + 1) * stagnation_iteration {
                        break;
                    }

                    progress.progress.store(
                        usize::max(
                            (i - stagnation_iteration) * 100
                                / (SAMPLING_LENGTH * stagnation_iteration),
                            winners.len() * 100 / population.len(),
                        ),
                        Ordering::Relaxed,
                    );
                }
            }

            if progress.sigint.load(Ordering::Relaxed) {
                break;
            }
        }

        // tournament phase
        let (winner, _) = self
            .final_tournament
            .run(&winners, &mut rng, &score_weights);
        *progress.top.write().unwrap() = winner.clone();

        winner
    }
}
