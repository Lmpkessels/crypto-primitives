use crate::utils::{
    rotl, z
};

use crate::ripemd::{
    l_round_constants, r_round_constants, f_left, f_right
};

/// RIPEMD-160 compression function.
///
/// # Parameters
/// - `m`: Vector of 16-word (512-bit) message blocks in little-endian order.
///
/// # Description
/// - Processes each block using dual (left and right) rounds over 80 steps.
/// - Each round updates five 32-bit words (A-E / A'-E') using nonlinear 
//    functions and constants.
/// - All variables starting with left represent the left round (A-E).
/// - All variables starting with right represent the right round (A'-E').
/// - z is .wrapping_add() 2^32.
///
/// # Returns
/// A 160-bit digest represented as an array of five 32-bit words.
///
/// # Reference
/// Based on the RIPEMD-160 specification:
/// [KULeuven Report AB-9601](https://homes.esat.kuleuven.be/~bosselae/ripemd160/pdf/AB-9601/AB-9601.pdf)
fn ripemd160(m: Vec<[u32; 16]>) -> [u32; 5] {
    /// Left round index order for message.
    pub const LEFT_ORDER: [u32; 80] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        7, 4, 13, 1, 10, 6, 15, 3, 12, 0, 9, 5, 2, 14, 11, 8,
        3, 10, 14, 4, 9, 15, 8, 1, 2, 7, 0, 6, 13, 11, 5, 12,
        1, 9, 11, 10, 0, 8, 12, 4, 13, 3, 7, 15, 14, 5, 6, 2,
        4, 0, 5, 9, 7, 12, 2, 10, 14, 1, 3, 8, 11, 6, 15, 13,
    ];
    
    /// Ranges left round rotations.
    pub const LEFT_ROTL: [u32; 80] = [
        11, 14, 15, 12, 5, 8, 7, 9, 11, 13, 14, 15, 6, 7, 9, 8,
        7, 6, 8, 13, 11, 9, 7, 15, 7, 12, 15, 9, 11, 7, 13, 12,
        11, 13, 6, 7, 14, 9, 13, 15, 14, 8, 13, 6, 5, 12, 7, 5,
        11, 12, 14, 15, 14, 15, 9, 8, 9, 14, 5, 6, 8, 6, 5, 12,
        9, 15, 5, 11, 6, 8, 13, 12, 5, 12, 13, 14, 11, 8, 5, 6,
    ];
    
    /// Right round index order for message.
    pub const RIGHT_ORDER: [u32; 80] = [
        5, 14, 7, 0, 9, 2, 11, 4, 13, 6, 15, 8, 1, 10, 3, 12,
        6, 11, 3, 7, 0, 13, 5, 10, 14, 15, 8, 12, 4, 9, 1, 2,
        15, 5, 1, 3, 7, 14, 6, 9, 11, 8, 12, 2, 10, 0, 4, 13,
        8, 6, 4, 1, 3, 11, 15, 0, 5, 12, 2, 13, 9, 7, 10, 14,
        12, 15, 10, 4, 1, 5, 8, 7, 6, 2, 13, 14, 0, 3, 9, 11
    ];
        
    /// Ranges right round rotations.
    pub const RIGHT_ROTL: [u32; 80] = [
        8, 9, 9, 11, 13, 15, 15, 5, 7, 7, 8, 11, 14, 14, 12, 6,
        9, 13, 15, 7, 12, 8, 9, 11, 7, 7, 12, 7, 6, 15, 13, 11,
        9, 7, 15, 11, 8, 6, 6, 14, 12, 13, 5, 14, 13, 13, 7, 5,
        15, 5, 8, 11, 14, 14, 6, 14, 6, 9, 12, 9, 12, 5, 15, 8,
        8, 5, 12, 9, 12, 5, 14, 6, 8, 13, 6, 5, 15, 13, 11, 11
    ];

    let mut digest = [0u32; 5];
    
    // Initial hexdecimal values.
    let mut h0: u32 = 0x67452301;
    let mut h1: u32 = 0xefcdab89;
    let mut h2: u32 = 0x98badcfe; 
    let mut h3: u32 = 0x10325476;
    let mut h4: u32 = 0xc3d2e1f0;
    
    // Access 16 32-bit words in message block.
    for x in &m {
        // Left round hexdecimal values.
        let mut left_a = h0;
        let mut left_b = h1;
        let mut left_c = h2;
        let mut left_d = h3;
        let mut left_e = h4;
        
        // Left round hexdecimal values.
        let mut right_a = h0;
        let mut right_b = h1;
        let mut right_c = h2;
        let mut right_d = h3;
        let mut right_e = h4;
        
        let mut left_temp;
        let mut right_temp;

        for j in 0..80 {
            left_temp = z(
                rotl(
                    z(
                        z(
                            z(
                                left_a, f_left(j, left_b, left_c, left_d)), 
                            x[LEFT_ORDER[j as usize] as usize]), 
                        l_round_constants(j)), 
                    LEFT_ROTL[j as usize]),
                left_e
            );

            // Update left hexdecimal values after each round.
            left_a = left_e;
            left_e = left_d;
            left_d = rotl(left_c, 10);
            left_c = left_b;
            left_b = left_temp;

            right_temp = z(
                rotl(
                    z(
                        z(
                            z(
                                right_a, f_right(j, right_b, right_c, right_d)), 
                            x[RIGHT_ORDER[j as usize] as usize]), 
                        r_round_constants(j)), 
                    RIGHT_ROTL[j as usize]),
                right_e, 
            );
            
            // Update left hexdecimal values after each round.
            right_a = right_e;
            right_e = right_d;
            right_d = rotl(right_c, 10);
            right_c = right_b;
            right_b = right_temp;
        }
        // Final message digestion.
        let t = z(z(h1, left_c), right_d);
        h1 = z(z(h2, left_d), right_e);
        h2 = z(z(h3, left_e), right_a);
        h3 = z(z(h4, left_a), right_b);
        h4 = z(z(h0, left_b), right_c);
        h0 = t;

        // Swap bytes to right order.
        digest = [
            h0.swap_bytes(), 
            h1.swap_bytes(), 
            h2.swap_bytes(), 
            h3.swap_bytes(), 
            h4.swap_bytes()
        ];
    }
    digest
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::padd_pars::{ little_endian_padd, little_endian_pars };

    #[test]
    fn ripemd160_empty_string() {
        let msg = b"";
        let padded = little_endian_padd(msg);
        let parsed = little_endian_pars(padded);
        
        let result = ripemd160(parsed);
        let expected = [
            0x9c1185a5, 0xc5e9fc54, 0x61280897, 0x7ee8f548, 0xb2258d31
        ];

        assert_eq!((result), (expected));
    }

    #[test]
    fn ripemd160_single_a() {
        let msg = b"a";
        let padded = little_endian_padd(msg);
        let parsed = little_endian_pars(padded);

        let result = ripemd160(parsed);
        let expected = [
            0x0bdc9d2d, 0x256b3ee9, 0xdaae347b, 0xe6f4dc83, 0x5a467ffe
        ];

        assert_eq!((result), (expected));
    }

    #[test]
    fn ripemd160_abc() {
        let msg = b"abc";
        let padded = little_endian_padd(msg);
        let parsed = little_endian_pars(padded);

        let result = ripemd160(parsed);
        let expected = [
            0x8eb208f7, 0xe05d987a, 0x9b044a8e, 0x98c6b087, 0xf15a0bfc
        ];

        assert_eq!((result), (expected));
    }

    #[test]
    fn ripemd160_long_message() {
        let msg = b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
        let padded = little_endian_padd(msg);
        let parsed = little_endian_pars(padded);

        let result = ripemd160(parsed);
        let expected = [
            0x12a05338, 0x4a9c0c88, 0xe405a06c, 0x27dcf49a, 0xda62eb2b
        ];

        assert_eq!((result), (expected));
    }   

    #[test]
    fn ripemd160_1million_as() {
        let msg = b"a".repeat(1_000_000);
        let padded = little_endian_padd(&msg);
        let parsed = little_endian_pars(padded);

        let result = ripemd160(parsed);
        let expected = [
            0x52783243, 0xc1697bdb, 0xe16d37f9, 0x7f68f083, 0x25dc1528
        ];

        assert_eq!((result), (expected));
    }
}
