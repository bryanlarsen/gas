use crate::chromosone::{self, Chromosone};

use array_init::array_init;

use super::FitnessFunction;

/**

In a scheduling system, there are various types of shifts like overnight, weekend, holiday, etc.  Each employee would like to have a certain number of each type of shift.

This type of fitness function is genericized here by assigning each chromosone location (aka shift) a "color" (aka shift type), and then for each symbol (aka employee), count how many of each color is in the chromosone.   The negated absolute difference between each count and corresponding preference is a fitness score.  (negated so that 0 is the highest score).

Parameters:
  chromosone_colors: the color of each locus in a corresponding chromosone
  preferences: the preferred count each symbol has for each color.

*/

pub struct ColorCount {
    pub chromosone_colors: Vec<usize>,
    pub preferences: Vec<Vec<usize>>,
    pub ncolors: usize,
    pub color_names: &'static [&'static str],
}

impl ColorCount {
    /// see [`ColorCount`] docs
    pub fn new(
        ncolors: usize,
        chromosone_colors: Vec<usize>,
        preferences: Vec<Vec<usize>>,
        color_names: &'static [&'static str],
    ) -> ColorCount {
        assert_eq!(chromosone_colors.len(), chromosone::LENGTH);
        for prefs in preferences.iter() {
            assert_eq!(ncolors, prefs.len());
        }
        ColorCount {
            ncolors,
            chromosone_colors,
            preferences,
            color_names,
        }
    }

    fn count(&self, chromosone: &Chromosone) -> [Vec<usize>; chromosone::NSYMS] {
        let mut counts: [Vec<usize>; chromosone::NSYMS] =
            array_init(|_| vec![0usize; self.ncolors]);
        for (i, sym) in chromosone.iter().enumerate() {
            let color = self.chromosone_colors[i];
            counts[*sym as usize][color] += 1;
        }
        counts
    }
}

impl FitnessFunction for ColorCount {
    fn nscores(&self) -> usize {
        self.ncolors * chromosone::NSYMS
    }

    fn weights(&self) -> Vec<f64> {
        vec![1.0; self.nscores()]
    }

    fn run(&self, chromosone: &Chromosone) -> Vec<f64> {
        let mut scores = Vec::<f64>::with_capacity(self.nscores());
        let counts = self.count(chromosone);

        for m in 0..chromosone::NSYMS {
            for n in 0..self.ncolors {
                scores.push(-(counts[m][n].abs_diff(self.preferences[m][n]) as f64))
            }
        }

        scores
    }

    fn describe(&self, chromosone: &Chromosone) -> Vec<String> {
        let mut descriptions = Vec::<String>::with_capacity(chromosone::NSYMS);
        let counts = self.count(chromosone);

        for m in 0..chromosone::NSYMS {
            descriptions.push(
                (0..self.ncolors)
                    .map(|n| {
                        format!(
                            "{} {}-{}",
                            self.color_names[n], counts[m][n], self.preferences[m][n]
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(", "),
            );
        }

        descriptions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_count() {
        let cc = ColorCount::new(
            2,
            vec![0, 1, 0, 1, 0],
            vec![vec![1, 1], vec![0, 0], vec![2, 2]],
            &["weekday", "weekend"],
        );
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
