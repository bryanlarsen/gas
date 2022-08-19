pub const NSYMS: usize = 15;
pub const LENGTH: usize = 119;
pub const NCOLORS: usize = 3;
pub const MAX_WEIGHT: usize = 3;

pub const COLOR_PREFS: [&[usize]; NSYMS] = [
    &[6, 6, 2],  // 9
    &[6, 0, 0],  // 18
    &[4, 2, 1],  // 26
    &[6, 0, 1],  // 29
    &[4, 2, 0],  // 31
    &[9, 3, 2],  // 36
    &[2, 1, 0],  // 37
    &[4, 2, 0],  // 38
    &[0, 6, 0],  // 39
    &[2, 1, 2],  // 40
    &[6, 3, 2],  // 41
    &[4, 2, 0],  // 42
    &[4, 2, 0],  // 43
    &[4, 2, 0],  // 44
    &[0, 12, 2], // 45
];

pub const CHROMOSONE_COLORS: [usize; LENGTH] = [
    0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0,
    2, 2, 2, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0,
    0, 0, 2, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 2, 2, 2, 1, 1, 1, 1, 2, 2, 2,
];

pub const WEIGHTS: [[usize; LENGTH]; NSYMS] = [
    [
        0, 0, 2, 2, 2, 1, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 2, 2, 1, 0, 0, 0, 0, 2, 2, 1, 0, 0, 0,
        0, 1, 2, 2, 2, 0, 0, 0, 1, 2, 1, 0, 0, 0, 2, 0, 0, 1, 0, 0, 0, 0, 1, 2, 1, 0, 0, 0, 2, 0,
        2, 1, 0, 0, 0, 2, 1, 2, 1, 0, 0, 0, 2, 2, 0, 1, 0, 0, 0, 1, 0, 2, 1, 0, 0, 0, 2, 2, 2, 1,
        0, 0, 0, 2, 2, 2, 1, 0, 0, 0, 2, 2, 2, 1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ], // 9
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0,
        1, 2, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1, 0, 1, 2,
        0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 2, 0, 0,
        0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ], // 18
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0, 1,
        0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1,
        1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 0,
        1, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ], // 26
    [
        1, 0, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1,
        0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0,
        1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1,
    ], // 29
    [
        0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0,
        0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0,
    ], // 31
    [
        0, 0, 0, 0, 0, 3, 1, 1, 0, 0, 0, 3, 3, 1, 1, 1, 1, 0, 3, 0, 0, 0, 0, 0, 0, 3, 3, 1, 1, 1,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 3, 3, 1, 1, 1, 1, 0,
        3, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3,
        1, 1, 1, 1, 0, 3, 0, 0, 0, 0, 0, 0, 0, 3, 1, 1, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0,
    ], // 36
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 2, 1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
        0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 1, 0, 0, 0, 0, 0, 2,
        0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 2, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ], // 37
    [
        0, 2, 1, 0, 0, 0, 2, 0, 2, 1, 0, 0, 0, 2, 0, 2, 1, 0, 0, 0, 2, 0, 2, 0, 0, 0, 0, 2, 0, 0,
        1, 0, 0, 0, 0, 0, 2, 1, 0, 0, 0, 2, 0, 2, 1, 0, 0, 0, 2, 0, 2, 1, 0, 0, 0, 0, 0, 2, 1, 0,
        0, 0, 2, 0, 2, 1, 0, 0, 0, 2, 0, 0, 1, 0, 0, 0, 2, 0, 2, 1, 0, 0, 0, 0, 0, 2, 1, 0, 0, 0,
        2, 0, 2, 1, 0, 0, 0, 2, 0, 2, 1, 0, 0, 0, 2, 0, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ], // 38
    [
        0, 0, 0, 2, 3, 3, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 2, 2, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 1, 3, 2, 3, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ], // 39
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ], // 40
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ], // 41
    [
        0, 2, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 2,
        0, 0, 0, 1, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 2, 0, 0,
        0, 1, 0, 0, 2, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 2, 0, 0, 0, 1, 0, 0, 2, 0, 0, 0, 1,
        0, 0, 2, 0, 0, 0, 1, 0, 0, 2, 0, 0, 0, 1, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ], // 42
    [
        0, 2, 1, 0, 0, 0, 0, 0, 3, 1, 0, 0, 0, 0, 0, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 3, 1, 1, 0, 0, 2, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0,
        0, 0, 3, 0, 1, 1, 0, 0, 0, 3, 0, 1, 1, 0, 0, 0, 3, 0, 1, 1, 0, 0, 0, 3, 0, 1, 1, 0, 0, 0,
        1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ], // 43
    [
        0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 3, 0, 0, 3, 0, 0, 0, 0, 3, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 3, 0, 0,
        3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ], // 44
    [
        1, 1, 2, 0, 0, 0, 0, 1, 1, 2, 0, 0, 0, 0, 1, 1, 2, 1, 2, 2, 0, 1, 1, 2, 1, 2, 2, 0, 1, 1,
        0, 1, 1, 1, 2, 1, 1, 2, 1, 2, 2, 0, 1, 1, 0, 1, 2, 2, 0, 1, 1, 2, 1, 2, 2, 0, 1, 1, 2, 1,
        2, 2, 0, 1, 1, 2, 1, 2, 2, 0, 1, 1, 2, 1, 2, 2, 0, 1, 1, 2, 1, 2, 2, 0, 1, 1, 2, 1, 2, 2,
        0, 1, 1, 2, 1, 2, 2, 0, 1, 1, 2, 1, 2, 2, 0, 1, 1, 2, 1, 0, 0, 2, 2, 1, 1, 1, 0, 2, 2,
    ], // 45
];

pub const INVALID_POSITIONS: [[bool; LENGTH]; NSYMS] = [
    [
        false, false, false, false, false, false, false, false, false, false, false, true, false,
        true, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, true, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, true, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, true, true, true, true, true, true, true, true, true,
        true,
    ], // 9
    [
        true, true, true, true, true, true, true, true, true, false, false, true, true, false,
        true, true, false, false, true, true, true, true, true, false, false, true, true, true,
        true, true, false, false, true, true, true, true, true, false, false, true, true, false,
        false, true, false, false, true, true, true, true, true, false, false, true, true, false,
        false, true, false, false, true, true, true, true, true, false, true, true, true, true,
        true, true, false, false, true, true, true, true, true, false, false, true, true, true,
        true, true, false, false, true, true, true, true, true, false, false, true, true, true,
        true, true, false, false, true, true, true, true, true, false, false, true, true, true,
        true, true, true, true, true, true, true,
    ], // 18
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, false, false, true, false, false, true, false, false, false, true, true, true,
        false, true, true, true, true, true, true, false, false, false, false, true, true, true,
        false, false, false, false, true, false, true, false, false, false, false, true, false,
        true, false, false, false, false, true, false, true, false, false, false, false, true,
        false, true, false, false, false, false, true, false, true, false, false, false, false,
        true, false, true, false, true, false, false, true, false, true, false, false, false,
        false, true, false, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true,
    ], // 26
    [
        false, true, true, false, false, false, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, true, true, false, false,
        false, false, false, false, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, true, false, false,
        false, false, false, false, false, false, false, false, false, false, false, true, false,
        true, true, true, false, false, false, false, false, false, false, false, false, true,
        false, false, false, false, false, false, false, false, false, false, false, false, false,
        false, false, false, true, true, false, false, false, false, false, false, false, false,
    ], // 29
    [
        true, true, true, true, true, true, true, false, true, true, false, true, true, true, true,
        true, true, true, true, true, true, false, true, true, false, true, true, true, true, true,
        true, false, false, false, false, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, false, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, false, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, false, true, false, false, false, false, false, false, true, true, true, true, true,
        true,
    ], // 31
    [
        true, true, true, true, true, false, false, false, true, true, true, false, false, false,
        false, false, false, true, false, true, true, true, true, true, true, false, false, false,
        false, false, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, false, true, true, true, true, true, true, false, false, false, false,
        false, false, true, false, true, true, true, true, true, true, false, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, false, false, false, false, false, false, true, false, true, true, true, true,
        true, true, true, false, false, false, true, true, true, true, false, false, true, true,
        true, true, true, true, true,
    ], // 36
    [
        true, true, true, true, true, true, true, true, true, true, true, false, true, true, true,
        true, true, false, false, true, true, true, true, true, false, true, true, true, true,
        true, true, true, true, false, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, false, false, true, true, true, true, true,
        false, true, true, true, true, true, true, true, false, true, true, true, true, true,
        false, false, false, true, true, true, true, false, true, true, true, true, true, true,
        true, false, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true,
    ], // 37
    [
        false, false, false, false, false, true, false, false, false, false, false, false, true,
        false, false, false, false, true, true, true, false, false, false, true, false, false,
        true, false, false, true, false, true, true, true, true, false, false, false, false, false,
        true, false, false, false, false, false, false, true, false, false, false, false, false,
        false, true, true, false, false, false, false, false, true, false, false, false, false,
        true, true, true, false, false, true, false, false, false, true, false, false, false,
        false, false, false, true, true, false, false, false, false, false, true, false, false,
        false, false, false, false, true, false, false, false, false, false, false, true, false,
        false, false, false, true, true, true, true, true, true, true, true, true, true, true,
    ], // 38
    [
        true, true, false, false, false, false, false, false, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, false, false, false, false, false, false, false, true, true, true, true, true,
        true, true, true, true, true, false, false, false, false, false, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true,
    ], // 39
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    ], // 40
    [
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    ], // 41
    [
        true, false, true, false, false, true, true, true, false, true, false, false, true, true,
        true, true, true, false, true, false, true, true, false, true, false, true, true, true,
        true, false, true, false, false, false, true, true, false, true, false, true, false, true,
        true, true, true, false, false, false, true, true, false, true, true, true, true, true,
        true, false, true, false, false, false, true, true, false, true, false, false, true, true,
        true, false, true, false, false, false, true, true, false, true, false, false, false, true,
        true, false, true, false, false, false, true, true, false, true, false, false, false, true,
        true, false, true, false, false, false, true, true, false, true, false, false, false,
        false, false, false, false, false, false, false, false,
    ], // 42
    [
        true, false, false, true, true, true, true, true, false, false, true, true, true, true,
        true, false, false, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, false, false, false, true, true, false, true,
        false, false, false, true, true, true, true, true, true, true, true, true, true, true,
        false, false, true, true, true, false, true, false, false, true, true, true, false, true,
        false, false, true, true, true, false, true, false, false, true, true, true, false, true,
        false, false, true, true, true, false, true, false, false, true, true, true, false, true,
        false, false, true, true, true, false, true, false, false, true, true, true, true, true,
        true, true, true, true, true, true,
    ], // 43
    [
        true, true, true, true, true, true, false, true, true, true, true, true, true, true, true,
        false, true, true, true, false, true, true, false, true, true, true, true, false, true,
        true, true, true, true, true, true, true, true, true, true, false, true, true, true, true,
        false, true, true, true, true, true, true, true, false, true, true, true, true, true, true,
        true, true, true, true, false, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, false, true, true, true, true, false, true,
        true, false, true, true, true, true, true, true, true, true, true, true, true, true, true,
        true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    ], // 44
    [
        false, false, false, true, true, true, true, false, false, false, true, true, true, true,
        false, false, false, false, false, false, true, false, false, false, false, false, false,
        true, false, false, true, false, false, false, false, false, false, false, false, false,
        false, true, false, false, true, false, false, false, true, false, false, false, false,
        false, false, true, false, false, false, false, false, false, true, false, false, false,
        false, false, false, true, false, false, false, false, false, false, true, false, false,
        false, false, false, false, true, false, false, false, false, false, false, true, false,
        false, false, false, false, false, true, false, false, false, false, false, false, true,
        false, false, false, false, true, true, false, false, false, false, false, true, false,
        false,
    ], // 45
];

pub const SYMBOL_NAMES: [&str; NSYMS] = [
    "9", "18", "26", "29", "31", "36", "37", "38", "39", "40", "41", "42", "43", "44", "45",
];

pub const LOCUS_NAMES: [&str; LENGTH] = [
    "10121", "10123", "10125", "10127", "10129", "10131", "10133", "10135", "10137", "10139",
    "10141", "10143", "10145", "10147", "10149", "10151", "10153", "10155", "10157", "10159",
    "10161", "10163", "10165", "10167", "10169", "10171", "10173", "10175", "10177", "10179",
    "10181", "10183", "10185", "10187", "10189", "10191", "10193", "10195", "10197", "10199",
    "10201", "10203", "10205", "10207", "10209", "10211", "10213", "10215", "10217", "10219",
    "10221", "10223", "10225", "10227", "10229", "10231", "10233", "10235", "10237", "10239",
    "10241", "10243", "10245", "10247", "10249", "10251", "10253", "10255", "10257", "10259",
    "10261", "10263", "10265", "10267", "10269", "10271", "10273", "10275", "10277", "10279",
    "10281", "10283", "10285", "10287", "10289", "10291", "10293", "10295", "10297", "10299",
    "10301", "10303", "10305", "10307", "10309", "10311", "10313", "10315", "10317", "10319",
    "10321", "10323", "10325", "10327", "10329", "10331", "10333", "10335", "10337", "10339",
    "10341", "10343", "10345", "10347", "10349", "10351", "10353", "10355", "10357",
];

pub const COLOR_NAMES: [&str; NCOLORS] = [&"weekday", &"weekend", &"stat"];

pub const DISTANCE_BEFORE: [usize; NSYMS] = [1, 12, 4, 6, 7, 9, 31, 28, 35, 179, 153, 2, 8, 21, 3];
