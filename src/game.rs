#[cfg(not(test))]
use crate::data::*;
#[cfg(test)]
use crate::test_data::*;

use crate::rando::*;

#[derive(Debug, PartialEq)]
pub enum LeftRight {
    Left,
    Right,
}

/// a single game between two candidates, given the scores for each candidate.
/// Each game consists of between TRIES_PER_GAME_MIN and TRIES_PER_GAME_MAX
/// points. Each try randomly selects a score position, and whichever candidate
/// has the higher score at that position wins the point. Whoever has the most
/// points wins. In the case of a tie, up to 10 tie breakers are attempted. If
/// it is still tied after 10 tie-breakers, the point goes to left.

pub fn game(left: &[f64], right: &[f64], rng: &mut dyn Rando) -> LeftRight {
    assert!(left.len() == right.len());

    let mut rpts = 0;
    let mut lpts = 0;
    let tries = rng.gen_range(TRIES_PER_GAME);

    for i in 0..tries + 10 {
        let pos = rng.gen_range(0..left.len());
        if left[pos] < right[pos] {
            rpts += 1;
        } else if left[pos] > right[pos] {
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
#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_game() {
        let mut r = MockRando::new();
        r.expect_gen_range()
            .with(predicate::eq(TRIES_PER_GAME))
            .times(1)
            .return_const(1usize);
        r.expect_gen_range()
            .with(predicate::eq(0..3usize))
            .times(1)
            .return_const(2usize);
        assert_eq!(
            LeftRight::Left,
            game(&[0.0, 0.1, 0.2], &[0.2, 0.1, 0.0], &mut r)
        );
    }

    #[test]
    fn test_game_tied() {
        let mut r = MockRando::new();
        r.expect_gen_range()
            .with(predicate::eq(TRIES_PER_GAME))
            .times(1)
            .return_const(1usize);
        r.expect_gen_range()
            .with(predicate::eq(0..3usize))
            .times(11)
            .return_const(2usize);
        assert_eq!(
            LeftRight::Left,
            game(&[0.0, 0.0, 0.0], &[0.0, 0.0, 0.0], &mut r)
        );
    }
}
