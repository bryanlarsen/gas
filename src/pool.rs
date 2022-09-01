use crate::candidate::Candidate;
use crate::gas::cycle::CycleProgress;
use crate::gas::Gas;
use crate::rando::Rando;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread;

/**
*  Run the algorithm on a set of populations simultaneously using multithreading.
*
*  Set sigint to true to stop the iterations manually.
*
**/
pub struct Pool {
    pub progresses: Vec<CycleProgress>,
    pub handles: Vec<thread::JoinHandle<Candidate>>,
    pub sigint: Arc<AtomicBool>,
}

impl Pool {
    pub fn new(gas: Arc<Gas>, nthreads: usize, sigint: Arc<AtomicBool>) -> Pool {
        let mut progresses = Vec::<CycleProgress>::with_capacity(nthreads);
        let mut handles = Vec::<thread::JoinHandle<Candidate>>::with_capacity(nthreads);

        for _ in 0..nthreads {
            let igas = gas.clone();
            let mut progress = CycleProgress::new(&igas, &sigint);
            progresses.push(progress.clone());

            handles.push(thread::spawn(move || {
                let mut population = Vec::<Candidate>::with_capacity(igas.population_size);
                let mut rng = Rando::new();
                for _ in 0..igas.population_size {
                    population.push(Candidate::new(&igas, &mut rng));
                }
                igas.cycle(&mut population, &mut progress, &mut rng)
            }));
        }
        Pool {
            progresses,
            handles,
            sigint,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.handles.iter().all(|h| h.is_finished())
    }

    pub fn winner(&mut self, gas: Arc<Gas>) -> Candidate {
        let mut rng = Rando::new();
        let winners: Vec<Candidate> = self.handles.drain(..).map(|h| h.join().unwrap()).collect();
        let (winner, _) = gas.final_tournament.run(&winners, &mut rng);
        winner
    }
}
