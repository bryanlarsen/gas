#[cfg(test)]
use crate::test_data::*;

use std::collections::VecDeque;

use crate::candidate::*;
use crate::game::*;
use crate::rando::*;
use crate::tournaments::*;

pub struct SingleElimination {}

impl SingleElimination {
    pub fn new() -> SingleElimination {
        SingleElimination {}
    }
}

impl Tournament for SingleElimination {
    fn run(&self, population: &Vec<Candidate>, rng: &mut dyn Rando) -> Vec<usize> {
        let mut remaining: VecDeque<usize> = VecDeque::with_capacity(population.len());
        let mut losers: Vec<usize> = Vec::with_capacity(population.len());

        for i in 0..population.len() {
            remaining.push_back(i);
        }
        rng.shuffle(remaining.make_contiguous());

        while remaining.len() >= 2 {
            let left = remaining.pop_front().unwrap();
            let right = remaining.pop_front().unwrap();

            match game(&population[left].scores, &population[right].scores, rng) {
                LeftRight::Left => {
                    remaining.push_back(left);
                    losers.push(right);
                }
                LeftRight::Right => {
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
        let (_, config) = configuration();
        let mut r = RealRando::new();
        let t = SingleElimination::new();
        assert_eq!(
            t.run(
                &vec![
                    Candidate::from_chromosone([1, 0, 1, 0, 1], &config),
                    Candidate::from_chromosone([0, 0, 1, 0, 1], &config)
                ],
                &mut r
            ),
            [1, 0]
        );
    }
}
