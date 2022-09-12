pub const NSYMS: usize = 15;
pub const LENGTH: usize = 119;
pub const NCOLORS: usize = 3;
pub const MAX_WEIGHT: usize = 3;

pub const COLOR_PREFS: [[usize; NCOLORS]; NSYMS] = [
    [6, 6, 2],  // 9
    [6, 0, 0],  // 18
    [4, 2, 1],  // 26
    [6, 0, 1],  // 29
    [4, 2, 0],  // 31
    [9, 3, 2],  // 36
    [2, 1, 0],  // 37
    [4, 2, 0],  // 38
    [0, 6, 0],  // 39
    [2, 1, 2],  // 40
    [6, 3, 2],  // 41
    [4, 2, 0],  // 42
    [4, 2, 0],  // 43
    [4, 2, 0],  // 44
    [0, 12, 2], // 45
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
    "Ariana Moore",
    "Madelyn Hall",
    "Karl Hoffman",
    "Daniella Brewer",
    "Natalia Porter",
    "Kara Bauer",
    "Gary Carr",
    "Alexis Mccormick",
    "Jennifer French",
    "Marie Spencer",
    "Madeline Walsh",
    "Parker Powers",
    "Francisco Burke",
    "Khalil Wise",
    "Brandon Williamson",
];

pub const LOCUS_NAMES: [&str; LENGTH] = [
    "2022-09-06",
    "2022-09-07",
    "2022-09-08",
    "2022-09-09",
    "2022-09-10",
    "2022-09-11",
    "2022-09-12",
    "2022-09-13",
    "2022-09-14",
    "2022-09-15",
    "2022-09-16",
    "2022-09-17",
    "2022-09-18",
    "2022-09-19",
    "2022-09-20",
    "2022-09-21",
    "2022-09-22",
    "2022-09-23",
    "2022-09-24",
    "2022-09-25",
    "2022-09-26",
    "2022-09-27",
    "2022-09-28",
    "2022-09-29",
    "2022-09-30",
    "2022-10-01",
    "2022-10-02",
    "2022-10-03",
    "2022-10-04",
    "2022-10-05",
    "2022-10-06",
    "2022-10-07",
    "2022-10-08",
    "2022-10-09",
    "2022-10-10",
    "2022-10-11",
    "2022-10-12",
    "2022-10-13",
    "2022-10-14",
    "2022-10-15",
    "2022-10-16",
    "2022-10-17",
    "2022-10-18",
    "2022-10-19",
    "2022-10-20",
    "2022-10-21",
    "2022-10-22",
    "2022-10-23",
    "2022-10-24",
    "2022-10-25",
    "2022-10-26",
    "2022-10-27",
    "2022-10-28",
    "2022-10-29",
    "2022-10-30",
    "2022-10-31",
    "2022-11-01",
    "2022-11-02",
    "2022-11-03",
    "2022-11-04",
    "2022-11-05",
    "2022-11-06",
    "2022-11-07",
    "2022-11-08",
    "2022-11-09",
    "2022-11-10",
    "2022-11-11",
    "2022-11-12",
    "2022-11-13",
    "2022-11-14",
    "2022-11-15",
    "2022-11-16",
    "2022-11-17",
    "2022-11-18",
    "2022-11-19",
    "2022-11-20",
    "2022-11-21",
    "2022-11-22",
    "2022-11-23",
    "2022-11-24",
    "2022-11-25",
    "2022-11-26",
    "2022-11-27",
    "2022-11-28",
    "2022-11-29",
    "2022-11-30",
    "2022-12-01",
    "2022-12-02",
    "2022-12-03",
    "2022-12-04",
    "2022-12-05",
    "2022-12-06",
    "2022-12-07",
    "2022-12-08",
    "2022-12-09",
    "2022-12-10",
    "2022-12-11",
    "2022-12-12",
    "2022-12-13",
    "2022-12-14",
    "2022-12-15",
    "2022-12-16",
    "2022-12-17",
    "2022-12-18",
    "2022-12-19",
    "2022-12-20",
    "2022-12-21",
    "2022-12-22",
    "2022-12-23",
    "2022-12-24",
    "2022-12-25",
    "2022-12-26",
    "2022-12-27",
    "2022-12-28",
    "2022-12-29",
    "2022-12-30",
    "2022-12-31",
    "2023-01-01",
    "2023-01-02",
];

pub const COLOR_NAMES: [&str; NCOLORS] = [&"weekday", &"weekend", &"stat"];

pub const DISTANCE_BEFORE: [Option<usize>; NSYMS] = [
    Some(1),
    Some(12),
    Some(4),
    Some(6),
    Some(7),
    Some(9),
    Some(31),
    Some(28),
    Some(35),
    Some(179),
    Some(153),
    Some(2),
    Some(8),
    Some(21),
    Some(3),
];
