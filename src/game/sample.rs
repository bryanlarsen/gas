#[cfg(test)]
use mockall::*;

use crate::candidate::Candidate;
use crate::game::*;

#[mockall_double::double]
use crate::rando::Rando;

#[derive(Debug, Clone)]
pub struct Sample<const N: usize, const NSYMS: usize> {
    pub tries_per_game: std::ops::Range<usize>,
}

/// a single game between two candidates Each game consists of a number of point
/// tries in range tries_per_game. Each try randomly selects a score position,
/// and whichever candidate has the higher score at that position wins the
/// point. Whoever has the most points wins. In the case of a tie, up to 10 tie
/// breakers are attempted. If it is still tied after 10 tie-breakers, the point
/// goes to left.
impl<const N: usize, const NSYMS: usize> Sample<N, NSYMS> {
    pub const fn new(tries_per_game: std::ops::Range<usize>) -> Self {
        Sample { tries_per_game }
    }
}

impl<const N: usize, const NSYMS: usize> Game<N, NSYMS> for Sample<N, NSYMS> {
    fn run(
        &self,
        left: &Candidate<N, NSYMS>,
        right: &Candidate<N, NSYMS>,
        rng: &mut Rando,
        score_weights: &Vec<f64>,
    ) -> LeftRight {
        if left.violations < right.violations {
            return LeftRight::Left;
        } else if left.violations > right.violations {
            return LeftRight::Right;
        }

        let mut rpts = 0;
        let mut lpts = 0;
        let tries = rng.gen_range(self.tries_per_game.clone());

        for i in 0..tries + 10 {
            let pos = rng.gen_range(0..left.scores.len());
            if left.scores[pos] < right.scores[pos] {
                rpts += 1;
            } else if left.scores[pos] > right.scores[pos] {
                lpts += 1;
            }
            // if either left or right is NaN we fall through with no winner, which is desired behaviour.

            if rpts != lpts && i + 1 >= tries {
                break;
            }
        }

        if lpts >= rpts {
            LeftRight::Left
        } else {
            LeftRight::Right
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    const TRIES_PER_GAME: std::ops::Range<usize> = 1usize..4;

    #[test]
    fn test_game() {
        let mut r = Rando::default();
        r.expect_gen_range()
            .with(predicate::eq(TRIES_PER_GAME))
            .times(1)
            .return_const(1usize);
        r.expect_gen_range()
            .with(predicate::eq(0..9))
            .times(1)
            .return_const(2usize);
        let g = Sample::<5, 3>::new(TRIES_PER_GAME);
        assert_eq!(
            LeftRight::Left,
            g.run(
                &Candidate {
                    chromosone: [0, 0, 0, 0, 0],
                    scores: vec![0.0, 0.1, 0.2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                    violations: 0,
                },
                &Candidate {
                    chromosone: [0, 0, 0, 0, 0],
                    scores: vec![0.2, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                    violations: 0,
                },
                &mut r,
                &vec![1.0; 9]
            )
        );
    }

    #[test]
    fn test_game_violations() {
        let mut r = Rando::default();
        let g = Sample::<5, 3>::new(TRIES_PER_GAME);
        assert_eq!(
            LeftRight::Right,
            g.run(
                &Candidate {
                    chromosone: [0, 0, 0, 0, 0],
                    scores: vec![0.0, 0.1, 0.2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                    violations: 2,
                },
                &Candidate {
                    chromosone: [0, 0, 0, 0, 0],
                    scores: vec![0.2, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                    violations: 1,
                },
                &mut r,
                &vec![1.0; 9],
            )
        );
    }

    #[test]
    fn test_game_tied() {
        let mut r = Rando::default();
        r.expect_gen_range()
            .with(predicate::eq(TRIES_PER_GAME))
            .times(1)
            .return_const(1usize);
        r.expect_gen_range()
            .with(predicate::eq(0..9))
            .times(11)
            .return_const(2usize);
        let g = Sample::<5, 3>::new(TRIES_PER_GAME);
        assert_eq!(
            LeftRight::Left,
            g.run(
                &Candidate {
                    chromosone: [0, 0, 0, 0, 0],
                    scores: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                    violations: 0,
                },
                &Candidate {
                    chromosone: [0, 0, 0, 0, 0],
                    scores: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                    violations: 0,
                },
                &mut r,
                &vec![1.0; 9],
            )
        );
    }
}
