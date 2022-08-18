use crate::candidate::Candidate;
use crate::game::{self, Game};
use crate::tournaments::{Tournament, single_elimination::SingleElimination};

#[mockall_double::double]
use crate::rando::Rando;

pub struct FullSeason<G: Game + Clone> {
    pub game: G,
}

impl<G: Game + Clone> FullSeason<G> {
    pub const fn new(game: G) -> FullSeason<G> {
        FullSeason { game }
    }
}

impl<G: Game + Clone> Tournament for FullSeason<G> {
    fn run(&self, population: &Vec<Candidate>, rng: &mut Rando) -> (Candidate, Vec<usize>) {
        let mut wins = vec![0usize; population.len()];

        for left in 0..population.len()-1 {
            for right in left+1..population.len() {
                match self.game.run(&population[left], &population[right], rng) {
                    game::LeftRight::Left => {
                        wins[left] += 1;
                    }
                    game::LeftRight::Right => {
                        wins[right] += 1;
                    }
                }
            }
        }

        let mut winners = Vec::<Candidate>::new();
        let max = wins.iter().fold(0usize, |max, w| max.max(*w));

        for i in 0..population.len() {
            if wins[i] == max {
                winners.push(population[i].clone());
            }
        }

        if winners.len() > 1 {
            (SingleElimination::new(self.game.clone()).run(&winners, rng).0, wins)
        } else {
            (winners[0], wins)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_season_tournament() {
        let pop = &vec![
            Candidate::from_chromosone([1, 0, 1, 0, 1]),
            Candidate::from_chromosone([0, 0, 1, 0, 1]),
        ];
        let mut r = Rando::default();
        let g = game::full::Full::new();
        let t = FullSeason::new(g);
        let (winner, weights) = t.run(&pop, &mut r);
        assert_eq!(weights, [1, 0]);
        Candidate::assert_eq(&winner, &pop[0]);
    }

}
