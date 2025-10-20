use crate::utils::{ ch, parity, maj };

/// Functions (Ch, Parity, Maj) for round 0..79, where 0 <= t <= 79. 
/// Parameter T it's argument is a loop variable.
pub fn f(t: u32, x: u32, y: u32, z: u32) -> u32 {
    match t {
        0..=19 => ch(x, y, z),
        20..=39 => parity(x, y, z),
        40..=59 => maj(x, y, z),
        60..=79 => parity(x, y, z),
        _ => unreachable!()
    }
}
