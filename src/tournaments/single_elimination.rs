use std::collections::VecDeque;

use crate::candidate::Candidate;
use crate::game::{self, Game};
use crate::tournaments::Tournament;

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
    fn run(&self, population: &Vec<Candidate>, rng: &mut Rando) -> (Candidate, Vec<usize>) {
        let mut remaining: VecDeque<usize> = VecDeque::with_capacity(population.len());
        let mut wins = vec![0usize; population.len()];

        for i in 0..population.len() {
            remaining.push_back(i);
        }
        rng.shuffle(remaining.make_contiguous());

        while remaining.len() >= 2 {
            let left = remaining.pop_front().unwrap();
            let right = remaining.pop_front().unwrap();

            // normally in the any given tournament round each player has the
            // same number of wins. However, if any player gets a bye because
            // the number of players isn't a power of 2 it won't be. To ensure
            // that the tournament is guaranteed to have the most number of wins
            // we have to give anybody who gets a "bye" a win for the bye.
            wins[left] = wins[right];

            match self.game.run(&population[left], &population[right], rng) {
                game::LeftRight::Left => {
                    remaining.push_back(left);
                    wins[left] += 1;
                }
                game::LeftRight::Right => {
                    remaining.push_back(right);
                    wins[right] += 1;
                }
            }
        }
        assert!(remaining.len() == 1);

        (population[remaining.pop_front().unwrap()].clone(), wins)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data::*;

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
        let (winner, weights) = t.run(&pop, &mut r);
        assert_eq!(weights, [1, 0]);
        Candidate::assert_eq(&winner, &pop[0]);
    }

    #[test]
    fn test_bye() {
        let config = configuration();
        let pop = &vec![
            Candidate::from_chromosone([1, 0, 1, 0, 1], &config),
            Candidate::from_chromosone([0, 0, 1, 0, 1], &config),
            Candidate::from_chromosone([0, 1, 2, 0, 1], &config),
        ];
        let mut r = Rando::default();
        r.expect_shuffle().times(1).return_const(());
        let g = game::full::Full::new();
        let t = SingleElimination::new(g);
        let (winner, weights) = t.run(&pop, &mut r);
        assert_eq!(weights, [1, 0, 2]);
        Candidate::assert_eq(&winner, &pop[2]);
    }
}
