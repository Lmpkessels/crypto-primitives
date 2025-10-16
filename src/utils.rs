/// Modular addition (x + y = (mod n))
///
/// Overflow is thrown away by the cast down to u32.
pub fn z(x: u32, y: u32) -> u32 {
    x.wrapping_add(y)
}

/// Logical right shift by n (pads with 0s at MSB).
pub fn shr(x: u32, n: u32) -> u32 {
    x >> n
}

/// Rotate right (ROTR).
/// 
/// Rotate x right by n bits within a 32-bit word (wraps bits around).
pub fn rotr(x: u32, n: u32) -> u32 {
    // Normalize 0..31.
    let n = n & 31;
    // Use the complement count within the 32-bit word.
    (x >> n) | (x << (32 - n))
}

/// Rotate left (ROTL).
///
/// Rotate x left by n bits within a 32-bit word (wraps bits around).
pub fn rotl(x: u32, n: u32) -> u32 {
    // Normalize 0..31.
    let n = n & 31;
    // Use the complement count within the 32-bit word.
    (x << n) | (x >> (32 - n))
}

/// Choose.
///
/// If 'x' is 1 then the output bit is 'y'.
/// If 'x' is 0 then the output bit is 'z'
pub fn ch(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (!(x) & z)
}

/// Majority.
///
/// If at least 2 of the 3 inputs are 1, output is 1.
/// Otherwise output is 0.
pub fn maj(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}

/// Big sigma 0 (Upper case).
///
/// Take unsigned 32-bit integer as argument for X.
/// Follow logical steps:
/// - Rotate 'x' right by 2 bits.
/// - Rotate 'x' right by 13 bits.
/// - Rotate 'x' right by 22 bits.
/// - Apply XOR bit-by-bit, on all 3 words.
pub fn big_sigma0(x: u32) -> u32 {
    rotr(x, 2) ^ rotr(x, 13) ^ rotr(x, 22)
}

/// Big sigma 1 (Upper case).
///
/// Take unsigned 32-bit integer as argument for X.
/// Follow logical steps:
/// - Rotate 'x' right by 6 bits.
/// - Rotate 'x' right by 11 bits.
/// - Rotate 'x' right by 25 bits.
/// - Apply XOR bit-by-bit, on all 3 words.
pub fn big_sigma1(x: u32) -> u32 {
    rotr(x, 6) ^ rotr(x, 11) ^ rotr(x, 25)
}

/// Small sigma 0 (Lower case).
///
/// Take unsigned 32-bit integer as argument for X.
/// Follow logical steps:
/// - Rotate 'x' right by 7 bits.
/// - Rotate 'x' right by 18 bits.
/// - Shift 'x' right by 3 bits.
/// - Apply XOR bit-by-bit, on all 3 words.
pub fn small_sigma0(x: u32) -> u32 {
    rotr(x, 7) ^ rotr(x, 18) ^ shr(x, 3)
}

/// Small sigma 1 (Lower case).
///
/// Take unsigned 32-bit integer as argument for X.
/// Follow logical steps:
/// - Rotate 'x' right by 17 bits.
/// - Rotate 'x' right by 19 bits.
/// - Shift 'x' right by 10 bits.
/// - Apply XOR bit-by-bit, on all 3 words.
pub fn small_sigma1(x: u32) -> u32 {
    rotr(x, 17) ^ rotr(x, 19) ^ shr(x, 10)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn computes_addition_modular_2_to_power_of_32_using_cast_down() {
        let x = 4294967295;
        let y = 4294967295;

        let result = z(x, y);
        let expected = 4294967294;

        assert_eq!((result), (expected));
    }

    #[test]
    fn shift_x_right_by_n() {
        let x = 312;
        let n = 2;

        let result = shr(x, n);
        let expected = 78;

        assert_eq!((result), (expected));
    }

    #[test]
    fn appends_bits_to_lsb_with_shifting_to_right() {
        let x = 15;
        let n = 3;

        let result = rotr(x, n);
        let expected = 3758096385;

        assert_eq!((result), (expected));
    }

    #[test]
    fn appends_bits_to_msb_with_shifting_to_left() {
        let x = 3758096385;
        let n = 3;

        let result = rotl(x, n);
        let expected = 15;

        assert_eq!((result), (expected));
    }

    #[test]
    fn ch_x_is_1_puts_out_y() {
        let x = 0xFFFFFFFF;
        let y = 0xAAAA_AAAA;
        let z = 0x5555_5555;
        let result = ch(x, y, z);
        let expected = y;

        assert_eq!((result), (expected));
    }

    #[test]
    fn ch_x_is_0_puts_out_z() {
        let x = 0x000000000;
        let y = 0xAAAA_AAAA;
        let z = 0x5555_5555;

        let result = ch(x, y, z);
        let expected = z;

        assert_eq!((result), (expected));
    }

    #[test]
    fn maj_computes_1_when_only_two_inputs_are_1() {
        let x = 0xFFFFFFFF;
        let y = 0xFFFFFFFF;
        let z = 0x00000000;
        
        let expected = 0xFFFFFFFF;
        let result = maj(x, y, z);

        assert_eq!((result), (expected));
    }

    #[test]
    fn maj_computes_0_when_only_one_input_is_1() {
        let x = 0xFFFFFFFF;
        let y = 0x00000000;
        let z = 0x00000000;

        let result = maj(x, y, z);
        let expected = 0x00000000;

        assert_eq!(result, expected);
    }

    #[test]
    fn big_sigma0_computes_expected_when_x_is_75() {
        let x = 75;

        let result = big_sigma0(x);
        let expected = rotr(75, 2) ^ rotr(75, 13) ^ rotr(75, 22);

        assert_eq!((result), (expected));
    }

    #[test]
    fn big_sigma1_computes_expected_when_x_is_3() {
        let x = 3;

        let result = big_sigma1(x);
        let expected = rotr(3, 6) ^ rotr(3, 11) ^ rotr(3, 25);

        assert_eq!((result), (expected));
    }

    #[test]
    fn small_sigma0_computes_expected_when_x_is_23() {
        let x = 23;

        let result = small_sigma0(x);
        let expected = rotr(23, 7) ^ rotr(23, 18) ^ shr(23, 3);

        assert_eq!((result), (expected));
    }

    #[test]
    fn small_sigma1_computes_expected_when_x_is_64() {
        let x = 64;

        let result = small_sigma1(x);
        let expected = rotr(64, 17) ^ rotr(64, 19) ^ shr(64, 10);

        assert_eq!((result), (expected));
    }
}   