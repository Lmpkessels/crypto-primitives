use crate::sha3::rc_func;

// Lota: to modify the bits of Lane(0, 0) in a manner depending on the round
// index Ir.
pub fn iota_func(a: &[[u64; 5]; 5], ir: usize) -> [[u64; 5]; 5] {
    let mut a_: [[u64; 5]; 5] = *a;
    let mut rc = 0u64;

    for j in 0..7 {
        if rc_func(j as u64 + 7 * ir as u64) != 0 {
            rc ^= 1u64 << ((1 << j) -1);
        }
    }

    a_[0][0] ^= rc;

    a_
}