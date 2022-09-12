use crate::candidate::Candidate;
use crate::game::{self, Game};
use crate::tournaments::{single_elimination::SingleElimination, Tournament};

#[mockall_double::double]
use crate::rando::Rando;

pub struct FullSeason<G: Game<N, NSYMS> + Clone, const N: usize, const NSYMS: usize> {
    pub game: G,
}

impl<G: Game<N, NSYMS> + Clone, const N: usize, const NSYMS: usize> FullSeason<G, N, NSYMS> {
    pub const fn new(game: G) -> FullSeason<G, N, NSYMS> {
        FullSeason { game }
    }
}

impl<G: Game<N, NSYMS> + Clone, const N: usize, const NSYMS: usize> Tournament<N, NSYMS>
    for FullSeason<G, N, NSYMS>
{
    fn run(
        &self,
        population: &Vec<Candidate<N, NSYMS>>,
        rng: &mut Rando,
        score_weights: &Vec<f64>,
    ) -> (Candidate<N, NSYMS>, Vec<usize>) {
        let mut wins = vec![0usize; population.len()];

        for left in 0..population.len() - 1 {
            for right in left + 1..population.len() {
                match self
                    .game
                    .run(&population[left], &population[right], rng, score_weights)
                {
                    game::LeftRight::Left => {
                        wins[left] += 1;
                    }
                    game::LeftRight::Right => {
                        wins[right] += 1;
                    }
                }
            }
        }

        let mut winners = Vec::<Candidate<N, NSYMS>>::new();
        let max = wins.iter().fold(0usize, |max, w| max.max(*w));

        // each player has win probability of wins/popsize.   That means ELO delta between 1/popsize and (popsize-1)/popsize probabilities is log10(popsize)*400.  Center on 1000 to make that average ELO.  Clamp popsize to 10000 so that ELO can't go below 0.
        let elo_spread = 400.0 * f64::log10(population.len() as f64);
        let elo_delta = elo_spread / population.len() as f64;
        let elo_base = 1000 - (elo_spread / 2.0) as usize;

        for i in 0..population.len() {
            if wins[i] == max {
                winners.push(population[i].clone());
            }

            wins[i] = elo_base + (elo_delta * (wins[i] as f64 + 0.5)) as usize;
        }

        if winners.len() > 1 {
            (
                SingleElimination::new(self.game.clone())
                    .run(&winners, rng, score_weights)
                    .0,
                wins,
            )
        } else {
            (winners[0].clone(), wins)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gas::Gas;

    #[test]
    fn test_season_tournament() {
        let gas = Gas::dut();
        let pop = &vec![
            Candidate::from_chromosone(&gas, [1, 0, 1, 0, 1]),
            Candidate::from_chromosone(&gas, [0, 0, 1, 0, 1]),
        ];
        let mut r = Rando::default();
        let g = game::full::Full::new();
        let t = FullSeason::new(g);
        let (winner, weights) = t.run(&pop, &mut r, &vec![1.0; 9]);
        assert_eq!(weights, [1030, 970]);
        Candidate::assert_eq(&winner, &pop[0]);
    }
}
