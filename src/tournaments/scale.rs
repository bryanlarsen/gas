use crate::candidate::Candidate;
use crate::tournaments::Tournament;

#[mockall_double::double]
use crate::rando::Rando;

/** This tournament runs a different tournament and scales the weights returned from the other tournament
*/
pub struct Scale<T: Tournament> {
    pub tournament: T,
    pub constant: usize,
    pub scale: f64,
    pub exponent: f64,
}

impl<T: Tournament> Scale<T> {
    #[cfg_attr(test, allow(dead_code))]
    pub const fn new(tournament: T, constant: usize, scale: f64, exponent: f64) -> Self {
        Self {
            tournament,
            constant,
            scale,
            exponent,
        }
    }
}

fn scale(weights: &mut [usize], constant: usize, scale: f64, exponent: f64) {
    for i in 0..weights.len() {
        let mut weight = weights[i] as f64;
        weight = weight.powf(exponent) * scale;
        weights[i] = constant + weight.round() as usize;
    }
}

impl<T: Tournament> Tournament for Scale<T> {
    fn run(
        &self,
        population: &Vec<Candidate>,
        rng: &mut Rando,
        score_weights: &Vec<f64>,
    ) -> (Candidate, Vec<usize>) {
        let (winner, mut weights) = self.tournament.run(population, rng, score_weights);
        scale(&mut weights, self.constant, self.scale, self.exponent);
        (winner, weights)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale() {
        let mut weights = [0, 1, 2, 3];
        scale(&mut weights, 1, 1.5, 2.0);
        assert_eq!(weights, [1, 3, 7, 15]);
    }
}
