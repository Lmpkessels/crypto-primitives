pub fn k(t: u32) -> u32 {
    match t {
        0..=19 => 0x5a827999,
        20..=39 => 0x6ed9eba1,
        40..=59 => 0x8f1bbcdc,
        60..=79 => 0xca62c1d6,
        _ => unreachable!()
    }
}