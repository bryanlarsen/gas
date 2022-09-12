use super::Constraint;
use crate::chromosone::Gene;

/**

This constraint allows the specification of which positions in the chromosone are invalid for certain symbols.

 */

pub struct InvalidPosition<const N: usize, const NSYMS: usize> {
    pub invalid_positions: Vec<Vec<bool>>, // FIXME [[bool; N]; NSYMS]
}

impl<const N: usize, const NSYMS: usize> InvalidPosition<N, NSYMS> {
    pub const fn new(invalid_positions: Vec<Vec<bool>>) -> InvalidPosition<N, NSYMS> {
        InvalidPosition { invalid_positions }
    }
}

impl<const N: usize, const NSYMS: usize> Constraint<N, NSYMS> for InvalidPosition<N, NSYMS> {
    fn run(&self, chromosone: &[Gene; N]) -> usize {
        let mut violations: usize = 0;
        for (i, g) in chromosone.iter().enumerate() {
            if self.invalid_positions[*g as usize][i] {
                violations += 1;
            }
        }
        violations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_position() {
        let c = InvalidPosition::<5, 3>::new(vec![
            vec![false, false, false, false, false],
            vec![false, true, false, true, false],
            vec![true, true, true, true, true],
        ]);
        assert_eq!(c.run(&[0, 0, 0, 0, 0]), 0);
        assert_eq!(c.run(&[0, 1, 0, 0, 0]), 1);
        assert_eq!(c.run(&[1, 1, 1, 1, 1]), 2);
        assert_eq!(c.run(&[2, 0, 2, 0, 2]), 3);
    }
}
