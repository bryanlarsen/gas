use std::collections::VecDeque;

use super::single_elimination::SingleElimination;
use super::Tournament;
use crate::candidate::Candidate;
use crate::game::Game;

#[mockall_double::double]
use crate::rando::Rando;

pub struct DoubleElimination<G: Game<N, NSYMS> + Clone, const N: usize, const NSYMS: usize> {
    pub game: G,
}

impl<G: Game<N, NSYMS> + Clone, const N: usize, const NSYMS: usize> DoubleElimination<G, N, NSYMS> {
    pub const fn new(game: G) -> DoubleElimination<G, N, NSYMS> {
        DoubleElimination { game }
    }
}

impl<G: Game<N, NSYMS> + Clone, const N: usize, const NSYMS: usize> Tournament<N, NSYMS>
    for DoubleElimination<G, N, NSYMS>
{
    fn run(
        &self,
        population: &Vec<Candidate<N, NSYMS>>,
        rng: &mut Rando,
        score_weights: &Vec<f64>,
    ) -> (Candidate<N, NSYMS>, Vec<usize>) {
        let mut remaining: VecDeque<usize> = VecDeque::with_capacity(population.len());
        let mut rating = vec![1000usize; population.len()];

        for i in 0..population.len() {
            remaining.push_back(i);
        }
        rng.shuffle(remaining.make_contiguous());

        SingleElimination::new(self.game.clone()).do_side(
            population,
            &mut remaining,
            &mut rating,
            200.0,
            rng,
            score_weights,
        );
        let winner = remaining.pop_front().unwrap();
        for i in 0..population.len() {
            if i != winner {
                remaining.push_back(i);
            }
        }
        rng.shuffle(remaining.make_contiguous());
        SingleElimination::new(self.game.clone()).do_side(
            population,
            &mut remaining,
            &mut rating,
            180.0,
            rng,
            score_weights,
        );

        (population[winner].clone(), rating)
    }
    /*
        let mut roundA: VecDeque<usize> = VecDeque::with_capacity(population.len());
        let mut roundB: VecDeque<usize> = VecDeque::with_capacity(population.len());
        let mut rating = vec![1000usize; population.len()];
        let mut kA = 200.0;
        let mut kB = 180.0;

        for i in 0..population.len() {
            roundA.push_back(i);
        }
        rng.shuffle(roundA.make_contiguous());

        while roundA.len() >= 2 || roundB.len() >= 2 {
            if roundA.len() >= 2 {
                let left = roundA.pop_front().unwrap();
                let right = roundA.pop_front().unwrap();

                match self
                    .game
                    .run(&population[left], &population[right], rng, score_weights)
                {
                    game::LeftRight::Left => {
                        roundA.push_back(left);
                        roundB.push_back(right);
                        (rating[left], rating[right]) = elo(kA, rating[left], rating[right]);
                    }
                    game::LeftRight::Right => {
                        roundA.push_back(right);
                        roundB.push_back(left);
                        (rating[right], rating[left]) = elo(kA, rating[right], rating[left]);
                    }
                    // the next round starts when there are only a power of two candidates remaining.
                    if roundA.len().count_ones() == 1 {
                        kA *= 0.9;
                    }
                }
            }

            if roundB.len() >= 2 {
                let left = roundB.pop_front().unwrap();
                let right = roundB.pop_front().unwrap();

                match self
                    .game
                    .run(&population[left], &population[right], rng, score_weights)
                {
                    game::LeftRight::Left => {
                        roundB.push_back(left);
                        (rating[left], rating[right]) = elo(k, rating[left], rating[right]);
                    }
                    game::LeftRight::Right => {
                        roundB.push_back(right);
                        (rating[right], rating[left]) = elo(k, rating[right], rating[left]);
                    }
                    // the next round starts when there are only a power of two candidates remaining.
                    if roundA.len().count_ones() == 1 {
                        kA *= 0.9;
                    }
                }
            }

        }
        assert!(remaining.len() == 1);

        (population[remaining.pop_front().unwrap()].clone(), rating)
    }  */
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game;
    use crate::gas::Gas;

    #[test]
    fn test_double_elimination_tournament() {
        let gas = Gas::dut();
        let pop = &vec![
            Candidate::from_chromosone(&gas, [1, 0, 1, 0, 1]),
            Candidate::from_chromosone(&gas, [0, 0, 1, 0, 1]),
        ];
        let mut r = Rando::default();
        r.expect_shuffle().times(2).return_const(());
        let g = game::full::Full::new();
        let t = DoubleElimination::new(g);
        let (winner, weights) = t.run(&pop, &mut r, &vec![1.0; 9]);
        assert_eq!(weights, [1100, 900]);
        Candidate::assert_eq(&winner, &pop[0]);
    }

    #[test]
    fn test_bye() {
        let gas = Gas::dut();
        let pop = &vec![
            Candidate::from_chromosone(&gas, [1, 0, 1, 0, 1]),
            Candidate::from_chromosone(&gas, [0, 0, 1, 0, 1]),
            Candidate::from_chromosone(&gas, [0, 1, 2, 0, 1]),
        ];
        let mut r = Rando::default();
        r.expect_shuffle().times(2).return_const(());
        let g = game::full::Full::new();
        let t = DoubleElimination::new(g);
        let (winner, weights) = t.run(&pop, &mut r, &vec![1.0; 9]);
        assert_eq!(weights, [1053, 832, 1115]);
        Candidate::assert_eq(&winner, &pop[2]);
    }
}
