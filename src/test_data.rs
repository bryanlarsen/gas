use crate::config::{Configuration, InitConfiguration};
use crate::fitness;
use crate::game;
use crate::mutation;
use crate::tournaments;

pub const NSYMS: usize = 3;
pub const LENGTH: usize = 5;
pub const TRIES_PER_GAME: std::ops::Range<usize> = 1usize..4usize;
pub const NSCORES: usize = NSYMS * 3;
//pub const POPSIZE: usize = 2;
pub const NCOLORS: usize = 2;
pub const TABOO_DISTANCE: usize = 1;

pub fn configuration() -> Configuration {
    Configuration::new(InitConfiguration {
        tournament: Box::new(tournaments::single_elimination::SingleElimination::new(
            game::sample::Sample::new(TRIES_PER_GAME),
        )),
        crossover: vec![],
        mutation: vec![(Box::new(mutation::mutate::Mutate::new(1)), 1)],
        fitness: vec![Box::new(fitness::distance::Distance::new(7))],
        constraint: vec![],
    })
}
