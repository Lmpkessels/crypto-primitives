// Finite field multiplication (xTimes).
fn xtimes(byte: u8) -> u8 {
    if byte & 0x80 != 0 {
        (byte << 1) ^ 0x1b
    } else {
        byte << 1
    }
}

// Galois field multiplication.
fn gf_mul(mut a: u8, mut b: u8) -> u8 {
    let mut result = 0;

    while b != 0 {
        if b & 1 != 0 {
            result ^= a;
        }

        a = xtimes(a);
        b >>= 1;
    }

    result
}

fn main() {
    let msb_is_1 = xtimes(128);
    println!("MSB is 1: {}", msb_is_1);

    let msb_is_0 = xtimes(0);
    println!("MSB is 0: {}", msb_is_0);

    println!("Galois Field Multiplication: {}", gf_mul(18, 19));
}