use crate::candidate::Candidate;
use crate::gas::cycle::CycleProgress;
use crate::gas::Gas;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::thread;

#[mockall_double::double]
use crate::rando::Rando;
/**
*  Run the algorithm on a set of populations simultaneously using multithreading.
*
*  Set sigint to true to stop the iterations manually.
*
**/
pub struct Pool<const N: usize, const NSYMS: usize> {
    pub progresses: Vec<CycleProgress<N, NSYMS>>,
    pub handles: Vec<thread::JoinHandle<Candidate<N, NSYMS>>>,
    pub sigint: Arc<AtomicBool>,
}

impl<const N: usize, const NSYMS: usize> Pool<N, NSYMS> {
    pub fn new(
        gas: Arc<Gas<N, NSYMS>>,
        nthreads: usize,
        sigint: Arc<AtomicBool>,
    ) -> Pool<N, NSYMS> {
        let mut progresses = Vec::<CycleProgress<N, NSYMS>>::with_capacity(nthreads);
        let mut handles = Vec::<thread::JoinHandle<Candidate<N, NSYMS>>>::with_capacity(nthreads);

        for _ in 0..nthreads {
            let igas = gas.clone();
            let mut progress = CycleProgress::new(&igas, &sigint);
            progresses.push(progress.clone());

            handles.push(thread::spawn(move || igas.cycle(&mut progress)));
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

    pub fn winner(&mut self, gas: Arc<Gas<N, NSYMS>>) -> Candidate<N, NSYMS> {
        let mut rng = Rando::new();
        let winners: Vec<Candidate<N, NSYMS>> =
            self.handles.drain(..).map(|h| h.join().unwrap()).collect();
        let (winner, _) = gas
            .final_tournament
            .run(&winners, &mut rng, &gas.fitness.weights());
        winner
    }
}
