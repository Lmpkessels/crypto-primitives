/// All functions work on unsigned 64-bit words.

/// Wrapping add (MOD 2^64).
pub fn z64(x: u64, y: u64) -> u64 {
    x.wrapping_add(y)
}

/// Shifts 'x' right by 'n'.
pub fn shr64(x: u64, n: u64) -> u64 {
    x >> n
}

/// Rotate right, shifts bits of 'x' outside of right scope LSB,
/// and appends at MSB (Wraps bits around).
pub fn rotr64(x: u64, n: u64) -> u64 {
    let n = n & 63;
    (x >> n) | (x << (64 - n))
}

/// Rotate left, shifts bits of 'x' outside of left scope MSB,
/// and appends at LSB (Wraps bits around).
pub fn rotl64(x: u64, n: u64) -> u64 {
    let n = n & 63;
    (x << n) | (x >> (64 - n))
}

/// Choise.
///
/// If 'x' is 1 then the output bit is 'y'.
/// If 'x' is 0 then the output bit is 'z' 
pub fn ch64(x: u64, y: u64, z: u64) -> u64 {
    (x & y) ^ (!(x) & z)
}

/// Majority.
///
/// If at least 2 of the 3 inputs are 1, output is 1.
/// Otherwise output is 0.
pub fn maj64(x: u64, y: u64, z: u64) -> u64 {
    (x & y) ^ (x & z) ^ (y & z)
}

/// Big sigma 0 (Upper case).
///
/// Take unsigned 32-bit integer as argument for X.
/// Follow logical steps:
/// - Rotate 'x' right by 28 bits.
/// - Rotate 'x' right by 32 bits.
/// - Rotate 'x' right by 39 bits.
/// - Apply XOR bit-by-bit, on all 3 words.
pub fn big_sigma0_64(x: u64) -> u64 {
    rotr64(x, 28) ^ rotr64(x, 34) ^ rotr64(x, 39)
}

/// Big sigma 1 (Upper case).
///
/// Take unsigned 32-bit integer as argument for X.
/// Follow logical steps:
/// - Rotate 'x' right by 14 bits.
/// - Rotate 'x' right by 18 bits.
/// - Rotate 'x' right by 41 bits.
/// - Apply XOR bit-by-bit, on all 3 words.
pub fn big_sigma1_64(x: u64) -> u64 {
    rotr64(x, 14) ^ rotr64(x, 18) ^ rotr64(x, 41)
}

/// Small sigma 0 (Lower case).
///
/// Take unsigned 32-bit integer as argument for X.
/// Follow logical steps:
/// - Rotate 'x' right by 1 bits.
/// - Rotate 'x' right by 8 bits.
/// - Shift 'x' right by 7 bits.
/// - Apply XOR bit-by-bit, on all 3 words.
pub fn small_sigma0_64(x: u64) -> u64 {
    rotr64(x, 1) ^ rotr64(x, 8) ^ shr64(x, 7)
}

/// Small sigma 1 (Lower case).
///
/// Take unsigned 32-bit integer as argument for X.
/// Follow logical steps:
/// - Rotate 'x' right by 19 bits.
/// - Rotate 'x' right by 61 bits.
/// - Shift 'x' right by 6 bits.
/// - Apply XOR bit-by-bit, on all 3 words.
pub fn small_sigma1_64(x: u64) -> u64 {
    rotr64(x, 19) ^ rotr64(x, 61) ^ shr64(x, 6)
}