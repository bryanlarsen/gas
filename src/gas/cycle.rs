use super::Gas;
use crate::candidate::Candidate;
use crate::chromosone;

#[mockall_double::double]
use crate::rando::Rando;

use std::sync::{
    atomic::{AtomicBool, AtomicIsize, AtomicUsize, Ordering},
    Arc, RwLock,
};

#[cfg_attr(test, allow(dead_code))]
/** Communication between a [cycle] running in a thread and the main thread. */
pub struct CycleProgress {
    /// out: the number of iterations of the GA that have been run
    pub iteration: Arc<AtomicUsize>,
    /// out: continually updated with a rounded integer of [Candidate#total_score] of the best candidate
    pub score: Arc<AtomicIsize>,
    /// out: continually updated with the number of constraint violations of the best candidate
    pub violations: Arc<AtomicUsize>,
    /// out: continually updated with progress, values 0-100.   Does not increment until stagnation is detected.
    pub progress: Arc<AtomicUsize>,
    /// out: copy of the top candidate
    pub top: Arc<RwLock<Candidate>>,
    /// in: SIGINT or similar.  if set, cycle will finish and exit ASAP
    pub sigint: Arc<AtomicBool>,
}

#[cfg_attr(test, allow(dead_code))]
impl CycleProgress {
    pub fn new(gas: &Gas, sigint: &Arc<AtomicBool>) -> CycleProgress {
        CycleProgress {
            iteration: Arc::new(AtomicUsize::new(0)),
            score: Arc::new(AtomicIsize::new(0)),
            violations: Arc::new(AtomicUsize::new(0)),
            progress: Arc::new(AtomicUsize::new(0)),
            top: Arc::new(RwLock::new(Candidate::from_chromosone(
                gas,
                [0; chromosone::LENGTH],
            ))),
            sigint: Arc::clone(&sigint),
        }
    }

    /// Arc::clone all the Arc's.
    pub fn clone(&self) -> CycleProgress {
        CycleProgress {
            iteration: Arc::clone(&self.iteration),
            score: Arc::clone(&self.score),
            violations: Arc::clone(&self.violations),
            progress: Arc::clone(&self.progress),
            top: Arc::clone(&self.top),
            sigint: Arc::clone(&self.sigint),
        }
    }

    pub fn eprint(&self) {
        eprint!(
            "[{}: {}/{} {}%]\t",
            self.iteration.load(Ordering::Relaxed),
            self.score.load(Ordering::Relaxed),
            self.violations.load(Ordering::Relaxed),
            self.progress.load(Ordering::Relaxed),
        );
    }
}

/**
 * Given a population, run multiple [generation::generation]'s of the algorithm until it stagnates and then return the winner.
 *
 * This function is designed to be run in a thread, it keeps the [CycleProgress]
 * structure updated which allows the function to be monitored on the fly.
 *
 * Algorithm:
 *
 * 1. Run the GA until both the score and number of violations has stagnated.
 * 2. Keep running the GA, doing a biased sampling of the winners until we've got enough samples or we've been doing this step for too long.
 * 3. Do a final tournament of the winners do get the grand winner, which we return.
 *
 */
impl Gas {
    #[cfg_attr(test, allow(dead_code))]
    pub fn cycle(
        &self,
        population: &mut Vec<Candidate>,
        progress: &mut CycleProgress,
        rng: &mut Rando,
    ) -> Candidate {
        // fast moving average of score.  Stagnation of score is defined as when this goes below the slow moving average.
        let mut ema99 = population[0].total_score();
        // slow moving average of score
        let mut ema999 = ema99;
        // number of violations of current winner
        let mut cur_violations = population[0].violations;
        // how long the number of violations has stayed stable.  Stagnation is defined as when this reaches VIOLATIONS_STAGNATION_THRESHOLD
        let mut n_cur_violations = 1;
        // the state
        let mut stagnating = false;
        // best score ever seen
        let mut best_score = 0f64;
        // a sampling of generation winners for the final tournament
        let mut winners = Vec::<Candidate>::with_capacity(population.len());
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

        // seed so on sigint it's not empty
        winners.push(population[0].clone());

        for i in 0..(2 << 20) {
            progress.iteration.store(i, Ordering::Relaxed);

            *population = self.generation(population, rng);

            let ts = population[0].total_score();
            progress.score.store(ts.round() as isize, Ordering::Relaxed);
            progress
                .violations
                .store(population[0].violations, Ordering::Relaxed);

            match progress.top.try_write() {
                Err(_) => (),
                Ok(mut l) => *l = population[0].clone(),
            }

            if !stagnating {
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
                    stagnating = true;
                    stagnation_iteration = i;
                    winners.push(population[0].clone());
                }
            } else {
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
                        (i - stagnation_iteration) * 100 / (SAMPLING_LENGTH * stagnation_iteration),
                        winners.len() * 100 / population.len(),
                    ),
                    Ordering::Relaxed,
                );
            }

            if progress.sigint.load(Ordering::Relaxed) {
                break;
            }
        }

        // tournament phase
        let (winner, _) = self.final_tournament.run(&winners, rng);
        *progress.top.write().unwrap() = winner.clone();

        winner
    }
}
