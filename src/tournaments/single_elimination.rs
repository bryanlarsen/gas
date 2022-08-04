#[cfg(test)]
use crate::test_data::*;

use std::collections::VecDeque;

use crate::candidate::*;
use crate::game::{self, Game};
use crate::tournaments::*;

#[mockall_double::double]
use crate::rando::Rando;

pub struct SingleElimination<G: Game> {
    pub game: G,
}

impl<G: Game> SingleElimination<G> {
    pub fn new(game: G) -> SingleElimination<G> {
        SingleElimination { game }
    }
}

impl<G: Game> Tournament for SingleElimination<G> {
    fn run(&self, population: &Vec<Candidate>, rng: &mut Rando) -> Vec<usize> {
        let mut remaining: VecDeque<usize> = VecDeque::with_capacity(population.len());
        let mut losers: Vec<usize> = Vec::with_capacity(population.len());

        for i in 0..population.len() {
            remaining.push_back(i);
        }
        rng.shuffle(remaining.make_contiguous());

        while remaining.len() >= 2 {
            let left = remaining.pop_front().unwrap();
            let right = remaining.pop_front().unwrap();
            match self.game.run(&population[left], &population[right], rng) {
                game::LeftRight::Left => {
                    remaining.push_back(left);
                    losers.push(right);
                }
                game::LeftRight::Right => {
                    remaining.push_back(right);
                    losers.push(left);
                }
            }
        }
        assert!(remaining.len() == 1);

        // only one remaining is the winner, aka the last loser
        losers.push(remaining.pop_front().unwrap());

        losers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_elimination_tournament() {
        let config = configuration();
        let pop = &vec![
            Candidate::from_chromosone([1, 0, 1, 0, 1], &config),
            Candidate::from_chromosone([0, 0, 1, 0, 1], &config),
        ];
        let mut r = Rando::default();
        r.expect_shuffle().times(1).return_const(());
        let g = game::full::Full::new();
        let t = SingleElimination::new(g);
        assert_eq!(t.run(&pop, &mut r), [1, 0]);
    }
}
