pub fn z64(x: u64, y: u64) -> u64 {
    x.wrapping_add(y)
}

pub fn shr64(x: u64, n: u64) -> u64 {
    x >> n
}

pub fn rotr64(x: u64, n: u64) -> u64 {
    let n = n & 63;
    (x >> n) | (x << (64 - n))
}

pub fn rotl64(x: u64, n: u64) -> u64 {
    let n = n & 63;
    (x << n) | (x >> (64 - n))
}

pub fn ch64(x: u64, y: u64, z: u64) -> u64 {
    (x & y) ^ (!(x) & z)
}

pub fn maj64(x: u64, y: u64, z: u64) -> u64 {
    (x & y) ^ (x & z) ^ (y & z)
}

pub fn big_sigma0_64(x: u64) -> u64 {
    rotr64(x, 28) ^ rotr64(x, 34) ^ rotr64(x, 39)
}

pub fn big_sigma1_64(x: u64) -> u64 {
    rotr64(x, 14) ^ rotr64(x, 18) ^ rotr64(x, 41)
}

pub fn small_sigma0_64(x: u64) -> u64 {
    rotr64(x, 1) ^ rotr64(x, 8) ^ shr64(x, 7)
}

pub fn small_sigma1_64(x: u64) -> u64 {
    rotr64(x, 19) ^ rotr64(x, 61) ^ shr64(x, 6)
}