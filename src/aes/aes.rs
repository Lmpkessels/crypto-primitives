// Finite field multiplication (xTimes).
fn xtimes(byte: u8) -> u8 {
    if byte & 0x80 != 0 {
        (byte << 1) ^ 0x1b
    } else {
        byte << 1
    }
}

fn main() {
    let msb_is_1 = xtimes(128);
    println!("MSB is 1: {}", msb_is_1);

    let msb_is_0 = xtimes(0);
    println!("MSB is 0: {}", msb_is_0);

    println!("{}", 0xaf & 0x80);
}