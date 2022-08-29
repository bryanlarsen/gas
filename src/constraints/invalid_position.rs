use crate::config::default::{LENGTH, NSYMS};

/**

This constraint allows the specification of which positions in the chromosone are invalid for certain symbols.

 */

pub struct InvalidPosition {
    pub invalid_positions: [[bool; LENGTH]; NSYMS],
}

impl InvalidPosition {
    pub const fn new(invalid_positions: [[bool; LENGTH]; NSYMS]) -> InvalidPosition {
        InvalidPosition { invalid_positions }
    }

    pub fn run(&self, chromosone: &[usize; LENGTH]) -> usize {
        let mut violations: usize = 0;
        for i in 0..LENGTH {
            if self.invalid_positions[chromosone[i]][i] {
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
        let c = InvalidPosition::new([
            [false, false, false, false, false],
            [false, true, false, true, false],
            [true, true, true, true, true],
        ]);
        assert_eq!(c.run(&[0, 0, 0, 0, 0]), 0);
        assert_eq!(c.run(&[0, 1, 0, 0, 0]), 1);
        assert_eq!(c.run(&[1, 1, 1, 1, 1]), 2);
        assert_eq!(c.run(&[2, 0, 2, 0, 2]), 3);
    }
}
