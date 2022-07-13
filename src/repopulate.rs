/*
#[cfg(not(test))]
use crate::data::*;
#[cfg(test)]
use crate::test_data::*;

#[cfg(test)]
use mockall::*;

use crate::candidate::*;
use crate::rando::*;
use crate::reproduction::*;

pub fn repopulate(population: &mut Vec<Candidate>, rng: &mut dyn Rando) {
    let len: usize = population.len();
    let mut weights = vec![0; len];
    for i in len / 2..len {
        weights[i] = i;
    }
    rng.init_weighted(&weights);
    let mutate = crate::reproduction::mutate::Mutate1::new();
    for i in 0..len / 2 {
        let candidate = rng.gen_weighted();
        population[i] = mutate.run(&population[candidate], rng);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repopulate() {
        let mut r = MockRando::new();
        let mut pop = &mut vec![
            Candidate::from_chromosone([0, 0, 1, 0, 1]),
            Candidate::from_chromosone([1, 0, 1, 0, 1]),
            Candidate::from_chromosone([0, 1, 2, 0, 1]),
        ];
        r.expect_init_weighted().times(1).return_const(());
        r.expect_gen_weighted().times(1).return_const(2usize);
        r.expect_gen_range()
            .with(predicate::eq(0..LENGTH))
            .times(1)
            .return_const(3usize);
        r.expect_gen_range()
            .with(predicate::eq(0..NSYMS))
            .times(1)
            .return_const(1usize);
        repopulate(&mut pop, &mut r);
        println!("{:?}", pop);
        assert_eq!(
            pop,
            &vec![
                Candidate::from_chromosone([0, 1, 2, 1, 1]),
                Candidate::from_chromosone([1, 0, 1, 0, 1]),
                Candidate::from_chromosone([0, 1, 2, 0, 1]),
            ]
        );
    }
}
*/
