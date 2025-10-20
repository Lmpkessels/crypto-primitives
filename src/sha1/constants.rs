/// Four constants (k) for round 0..79, where 0 <= t <= 79.
/// Parameter T it's argument is a loop variable.
pub fn k(t: u32) -> u32 {
    match t {
        0..=19 => 0x5a827999,
        20..=39 => 0x6ed9eba1,
        40..=59 => 0x8f1bbcdc,
        60..=79 => 0xca62c1d6,
        _ => unreachable!()
    }
}