use crate::config::config::{LENGTH, NSYMS};

use array_init::array_init;

/**

In a scheduling system, there are various types of shifts like overnight, weekend, holiday, etc.  Each employee would like to have a certain number of each type of shift.

This type of fitness function is genericized here by assigning each chromosone location (aka shift) a "color" (aka shift type), and then for each symbol (aka employee), count how many of each color is in the chromosone.   The negated absolute difference between each count and corresponding preference is a fitness score.  (negated so that 0 is the highest score).

Parameters:
  chromosone_colors: the color of each locus in a corresponding chromosone
  preferences: the preferred count each symbol has for each color.

*/

pub struct ColorCount {
    pub chromosone_colors: [usize; LENGTH],
    pub preferences: [&'static [usize]; NSYMS],
    pub ncolors: usize,
}

impl ColorCount {
    /// see [`ColorCount`] docs
    pub const fn new(
        ncolors: usize,
        chromosone_colors: [usize; LENGTH],
        preferences: [&'static [usize]; NSYMS],
    ) -> ColorCount {
        let mut i = 0;
        while i < preferences.len() {
            assert!(ncolors == preferences[i].len());
            i += 1;
        }
        ColorCount {
            ncolors,
            chromosone_colors,
            preferences,
        }
    }
    pub const fn nscores(&self) -> usize {
        self.ncolors * NSYMS
    }

    pub fn run(&self, chromosone: &[usize]) -> Vec<f64> {
        assert_eq!(chromosone.len(), self.chromosone_colors.len());
        let mut counts: [Vec<usize>; NSYMS] = array_init(|_| vec![0usize; self.ncolors]);
        let mut scores = Vec::<f64>::with_capacity(self.ncolors * NSYMS);

        for i in 0..chromosone.len() {
            let color = self.chromosone_colors[i];
            let sym = chromosone[i];
            counts[sym][color] += 1;
        }

        for m in 0..NSYMS {
            for n in 0..self.ncolors {
                scores.push(-(counts[m][n].abs_diff(self.preferences[m][n]) as f64))
            }
        }

        scores
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_count() {
        let cc = ColorCount::new(2, [0, 1, 0, 1, 0], [&[1, 1], &[0, 0], &[2, 2]]);
        let scores = cc.run(&[0, 0, 0, 1, 1]);
        // sym0 has 2 0's and 1 1, preferring one of each.
        // sym1 has 1 of each and prefers 0 of each.
        // sym2 has none of each and prefers 2 of each.
        assert_eq!(
            scores,
            vec![/*sym0*/ -1.0, 0.0, /*sym1*/ -1.0, -1.0, /*sym2*/ -2.0, -2.0]
        );
    }
}
