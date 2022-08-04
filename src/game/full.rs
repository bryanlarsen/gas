use crate::candidate::Candidate;
use crate::game::{Game, LeftRight};

#[mockall_double::double]
use crate::rando::Rando;

pub struct Full {}

/// A game that compares every score at the same position between candidate, all
/// metrics with equal weighting. The candidate with the least violations wins.
/// If that is equal, the candidate that is superior in the most scores wins. In
/// the case of a tie, winner is random.
impl Full {
    pub fn new() -> Full {
        Full {}
    }
}

impl Game for Full {
    fn run(&self, left: &Candidate, right: &Candidate, rng: &mut Rando) -> LeftRight {
        if left.violations < right.violations {
            return LeftRight::Left;
        } else if left.violations > right.violations {
            return LeftRight::Right;
        }

        let pts =
            left.scores
                .iter()
                .zip(right.scores.iter())
                .fold((0usize, 0usize), |pts, scores| {
                    if scores.0 > scores.1 {
                        (pts.0 + 1, pts.1)
                    } else if scores.1 > scores.0 {
                        (pts.0, pts.1 + 1)
                    } else {
                        pts
                    }
                });

        if pts.0 > pts.1 {
            LeftRight::Left
        } else if pts.1 > pts.0 {
            LeftRight::Right
        } else {
            if rng.gen_range(0..2) == 0 {
                LeftRight::Left
            } else {
                LeftRight::Right
            }
        }
    }
}

#[cfg(test)]
use mockall::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        let mut r = Rando::default();
        let g = Full::new();
        assert_eq!(
            LeftRight::Left,
            g.run(
                &Candidate {
                    chromosone: [0, 0, 0, 0, 0],
                    scores: [0.0, 0.1, 0.2, 0.1, 0.0, 0.0],
                    violations: 0,
                    iteration: 0,
                },
                &Candidate {
                    chromosone: [0, 0, 0, 0, 0],
                    scores: [0.2, 0.1, 0.0, 0.0, 0.0, 0.0],
                    violations: 0,
                    iteration: 0,
                },
                &mut r
            )
        );
    }

    #[test]
    fn test_game_violations() {
        let mut r = Rando::default();
        let g = Full::new();
        assert_eq!(
            LeftRight::Right,
            g.run(
                &Candidate {
                    chromosone: [0, 0, 0, 0, 0],
                    scores: [0.0, 0.1, 0.2, 0.0, 0.0, 0.0],
                    violations: 2,
                    iteration: 0,
                },
                &Candidate {
                    chromosone: [0, 0, 0, 0, 0],
                    scores: [0.2, 0.1, 0.0, 0.0, 0.0, 0.0],
                    violations: 1,
                    iteration: 0,
                },
                &mut r
            )
        );
    }

    #[test]
    fn test_game_tied() {
        let mut r = Rando::default();
        r.expect_gen_range()
            .with(predicate::eq(0..2))
            .times(1)
            .return_const(1usize);
        let g = Full::new();
        assert_eq!(
            LeftRight::Right,
            g.run(
                &Candidate {
                    chromosone: [0, 0, 0, 0, 0],
                    scores: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                    violations: 0,
                    iteration: 0,
                },
                &Candidate {
                    chromosone: [0, 0, 0, 0, 0],
                    scores: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                    violations: 0,
                    iteration: 0,
                },
                &mut r
            )
        );
    }
}
