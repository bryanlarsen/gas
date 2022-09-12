use std::collections::VecDeque;

use super::elo::elo;
use super::Tournament;
use crate::candidate::Candidate;
use crate::game::{self, Game};

#[mockall_double::double]
use crate::rando::Rando;

pub struct SingleElimination<G: Game<N, NSYMS>, const N: usize, const NSYMS: usize> {
    pub game: G,
}

impl<G: Game<N, NSYMS>, const N: usize, const NSYMS: usize> SingleElimination<G, N, NSYMS> {
    pub const fn new(game: G) -> SingleElimination<G, N, NSYMS> {
        SingleElimination { game }
    }

    /// the guts of the single elimination tournament, pulled out so it can be called for each side of a double elimination tournament.
    ///
    /// `remaining`: indices into `population` of the candidates that have not yet lost out of the tournament.  By taking 2 candidates from the top and inserting the winner into the bottom, the bye's for tournament sizes not a power of 2 are handled properly.
    /// `rating`: the ELO rating of candidates.   Indices in this vector match that of `population`
    /// `k`: the K factor for ELO rating.   This is reduced by 10% every round of the tournament.
    pub fn do_side(
        &self,
        population: &Vec<Candidate<N, NSYMS>>,
        remaining: &mut VecDeque<usize>,
        rating: &mut Vec<usize>,
        k: f64,
        rng: &mut Rando,
        score_weights: &Vec<f64>,
    ) -> () {
        let mut k = k;
        while remaining.len() >= 2 {
            let left = remaining.pop_front().unwrap();
            let right = remaining.pop_front().unwrap();

            match self
                .game
                .run(&population[left], &population[right], rng, score_weights)
            {
                game::LeftRight::Left => {
                    remaining.push_back(left);
                    (rating[left], rating[right]) = elo(k, rating[left], rating[right]);
                }
                game::LeftRight::Right => {
                    remaining.push_back(right);
                    (rating[right], rating[left]) = elo(k, rating[right], rating[left]);
                }
            }

            // the next round starts when there are a power of two candidates remaining.
            if remaining.len().count_ones() == 1 {
                k *= 0.9;
            }
        }
        assert!(remaining.len() == 1);
    }
}

impl<G: Game<N, NSYMS>, const N: usize, const NSYMS: usize> Tournament<N, NSYMS>
    for SingleElimination<G, N, NSYMS>
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

        self.do_side(
            population,
            &mut remaining,
            &mut rating,
            200.0,
            rng,
            score_weights,
        );

        (population[remaining.pop_front().unwrap()].clone(), rating)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gas::Gas;

    #[test]
    fn test_single_elimination_tournament() {
        let gas = Gas::dut();
        let pop = &vec![
            Candidate::from_chromosone(&gas, [1, 0, 1, 0, 1]),
            Candidate::from_chromosone(&gas, [0, 0, 1, 0, 1]),
        ];
        let mut r = Rando::default();
        r.expect_shuffle().times(1).return_const(());
        let g = game::full::Full::new();
        let t = SingleElimination::new(g);
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
        r.expect_shuffle().times(1).return_const(());
        let g = game::full::Full::new();
        let t = SingleElimination::new(g);
        let (winner, weights) = t.run(&pop, &mut r, &vec![1.0; 9]);
        assert_eq!(weights, [985, 900, 1115]);
        Candidate::assert_eq(&winner, &pop[2]);
    }
}
