//! Tables extracted from *Z80 EMUF* emulator from [k1.spdns.de](k1.spdns.de)
//! 0 clocks in tables means that instruction clocks count must me determinated
//! while executing

// NOTE: C2,CA,D2,DA,E2,EA,F2,FA are changed from 0 to 10
#[cfg_attr(rustfmt, rustfmt_skip)]
pub const CLOCKS_NORMAL: [u8; 256] = [
//  0   1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
    4, 10,  7,  6,  4,  4,  7,  4,  4, 11,  7,  6,  4,  4,  7,  4, // 00
    0, 10,  7,  6,  4,  4,  7,  4, 12, 11,  7,  6,  4,  4,  7,  4, // 10
    0, 10, 16,  6,  4,  4,  7,  4,  0, 11, 16,  6,  4,  4,  7,  4, // 20
    0, 10, 13,  6, 11, 11, 10,  4,  0, 11, 13,  6,  4,  4,  7,  4, // 30
    4,  4,  4,  4,  4,  4,  7,  4,  4,  4,  4,  4,  4,  4,  7,  4, // 40
    4,  4,  4,  4,  4,  4,  7,  4,  4,  4,  4,  4,  4,  4,  7,  4, // 50
    4,  4,  4,  4,  4,  4,  7,  4,  4,  4,  4,  4,  4,  4,  7,  4, // 60
    7,  7,  7,  7,  7,  7,  4,  7,  4,  4,  4,  4,  4,  4,  7,  4, // 70
    4,  4,  4,  4,  4,  4,  7,  4,  4,  4,  4,  4,  4,  4,  7,  4, // 80
    4,  4,  4,  4,  4,  4,  7,  4,  4,  4,  4,  4,  4,  4,  7,  4, // 90
    4,  4,  4,  4,  4,  4,  7,  4,  4,  4,  4,  4,  4,  4,  7,  4, // A0
    4,  4,  4,  4,  4,  4,  7,  4,  4,  4,  4,  4,  4,  4,  7,  4, // B0
    0, 10, 10, 10,  0, 11,  7, 11,  0, 10, 10,  0,  0, 17,  7, 11, // C0
    0, 10, 10, 11,  0, 11,  7, 11,  0,  4, 10, 11,  0,  4,  7, 11, // D0
    0, 10, 10, 19,  0, 11,  7, 11,  0,  4, 10,  4,  0,  0,  7, 11, // E0
    0, 10, 10,  4,  0, 11,  7, 11,  0,  6, 10,  4,  0,  4,  7, 11, // F0
];


#[cfg_attr(rustfmt, rustfmt_skip)]
pub const CLOCKS_CB: [u8; 256] = [
//  0   1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
    8,  8,  8,  8,  8,  8, 15,  8,  8,  8,  8,  8,  8,  8, 15,  8, // 00
    8,  8,  8,  8,  8,  8, 15,  8,  8,  8,  8,  8,  8,  8, 15,  8, // 10
    8,  8,  8,  8,  8,  8, 15,  8,  8,  8,  8,  8,  8,  8, 15,  8, // 20
    8,  8,  8,  8,  8,  8, 15,  8,  8,  8,  8,  8,  8,  8, 15,  8, // 30
    8,  8,  8,  8,  8,  8, 15,  8,  8,  8,  8,  8,  8,  8, 15,  8, // 40
    8,  8,  8,  8,  8,  8, 15,  8,  8,  8,  8,  8,  8,  8, 15,  8, // 50
    8,  8,  8,  8,  8,  8, 15,  8,  8,  8,  8,  8,  8,  8, 15,  8, // 60
    8,  8,  8,  8,  8,  8, 15,  8,  8,  8,  8,  8,  8,  8, 15,  8, // 70
    8,  8,  8,  8,  8,  8, 15,  8,  8,  8,  8,  8,  8,  8, 15,  8, // 80
    8,  8,  8,  8,  8,  8, 15,  8,  8,  8,  8,  8,  8,  8, 15,  8, // 90
    8,  8,  8,  8,  8,  8, 15,  8,  8,  8,  8,  8,  8,  8, 15,  8, // A0
    8,  8,  8,  8,  8,  8, 15,  8,  8,  8,  8,  8,  8,  8, 15,  8, // B0
    8,  8,  8,  8,  8,  8, 15,  8,  8,  8,  8,  8,  8,  8, 15,  8, // C0
    8,  8,  8,  8,  8,  8, 15,  8,  8,  8,  8,  8,  8,  8, 15,  8, // D0
    8,  8,  8,  8,  8,  8, 15,  8,  8,  8,  8,  8,  8,  8, 15,  8, // E0
    8,  8,  8,  8,  8,  8, 15,  8,  8,  8,  8,  8,  8,  8, 15,  8, // F0
];

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const CLOCKS_ED: [u8; 256] = [
//   0   1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
     8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8, // 00
     8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8, // 10
     8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8, // 20
     8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8, // 30
    12, 12, 15, 20,  8, 14,  8,  9, 12, 12, 15, 20,  8, 14,  8,  9, // 40
    12, 12, 15, 20,  8, 14,  8,  9, 12, 12, 15, 20,  8, 14,  8,  9, // 50
    12, 12, 15, 20,  8, 14,  8, 18, 12, 12, 15, 20,  8, 14,  8, 18, // 60
    12, 12, 15, 20,  8, 14,  8,  8, 12, 12, 15, 20,  8, 14,  8,  8, // 70
     8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8, // 80
     8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8, // 90
    16, 16, 16, 16,  8,  8,  8,  8, 16, 16, 16, 16,  8,  8,  8,  8, // A0
     0,  0,  0,  0,  8,  8,  8,  8,  0,  0,  0,  0,  8,  8,  8,  8, // B0
     8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8, // C0
     8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8, // D0
     8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8, // E0
     8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8,  8, // F0
];

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const CLOCKS_DD_FD: [u8; 256] = [
//  0   1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
    4,  4,  4,  4,  4,  4,  4,  4,  4,  15, 4,  4,  4,  4,  4,  4, // 00
    4,  4,  4,  4,  4,  4,  4,  4,  4,  15, 4,  4,  4,  4,  4,  4, // 10
    4,  14, 20, 10, 8,  8,  11, 4,  4,  15, 20, 10, 8,  8,  11, 4, // 20
    4,  4,  4,  4,  23, 23, 19, 4,  4,  15, 4,  4,  4,  4,  4,  4, // 30
    4,  4,  4,  4,  8,  8,  19, 4,  4,  4,  4,  4,  8,  8,  19, 4, // 40
    4,  4,  4,  4,  8,  8,  19, 4,  4,  4,  4,  4,  8,  8,  19, 4, // 50
    8,  8,  8,  8,  8,  8,  19, 8,  8,  8,  8,  8,  8,  8,  19, 8, // 60
    19, 19, 19, 19, 19, 19, 4, 19,  4,  4,  4,  4,  8,  8,  19, 4, // 70
    4,  4,  4,  4,  8,  8,  19, 4,  4,  4,  4,  4,  8,  8,  19, 4, // 80
    4,  4,  4,  4,  8,  8,  19, 4,  4,  4,  4,  4,  8,  8,  19, 4, // 90
    4,  4,  4,  4,  8,  8,  19, 4,  4,  4,  4,  4,  8,  8,  19, 4, // A0
    4,  4,  4,  4,  8,  8,  19, 4,  4,  4,  4,  4,  8,  8,  19, 4, // B0
    4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  0,  4,  4,  4,  4, // C0
    4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4,  4, // D0
    4,  14, 4,  23, 4,  15, 4,  4,  4,  8,  4,  4,  4,  4,  4,  4, // E0
    4,  4,  4,  4,  4,  4,  4,  4,  4,  10, 4,  4,  4,  4,  4,  4  // F0
];
#[cfg_attr(rustfmt, rustfmt_skip)]
pub const CLOCKS_DDCB_FDCB: [u8; 256] = [
//   0   1   2   3   4   5   6   7   8   9   A   B   C   D   E   F
    23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, // 00
    23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, // 10
    23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, // 20
    23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, // 30
    20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, // 40
    20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, // 50
    20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, // 60
    20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, 20, // 70
    23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, // 80
    23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, // 90
    23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, // A0
    23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, // B0
    23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, // C0
    23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, // D0
    23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, // E0
    23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, 23, // F0
];