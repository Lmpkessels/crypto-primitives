/// #Reference
/// All round constants, shifts, and message ordering are gained from:
/// [KULeuven Report AB-9601](https://homes.esat.kuleuven.be/~bosselae/ripemd160/pdf/AB-9601/AB-9601.pdf)

/// Left round constants.
///
/// Use range j for deciding which constant to take based on match.
pub fn l_round_constants(j: u32) -> u32 {
    match j {
        0..=15 => 0x00000000,
        16..=31 => 0x5a827999,
        32..=47 => 0x6ed9eba1,
        48..=63 => 0x8f1bbcdc,
        64..=79 => 0xa953fd4e,
        _ => unreachable!(),
    }
}

/// Right round constants.
///
/// Use range j for deciding which constant to take based on match.
pub fn r_round_constants(j: u32) -> u32 {
    match j {
        0..=15 => 0x50a28be6,
        16..=31 => 0x5c4dd124,
        32..=47 => 0x6d703ef3,
        48..=63 => 0x7a6d76e9,
        64..=79 => 0x00000000,
        _ => unreachable!(),
    }
}
