/// the length of a chromosone.
#[cfg(not(test))]
pub const LENGTH: usize = 119;
#[cfg(test)]
pub const LENGTH: usize = 5;

/// the number of genes.   Genes are stored as unsigned integers from 0..NSYMS.   Mapping these to meaningful real-world values is done in [schedule_data::SYMBOL_NAMES].
#[cfg(not(test))]
pub const NSYMS: usize = 15;
#[cfg(test)]
pub const NSYMS: usize = 3;

/** Gene is an unsized integer that can hold the range 0..NSYMS.   If you need to map them to ACGT or whatever, that can be done outside of this library **/
pub type Gene = u8;

pub type Chromosone = [Gene; LENGTH];
