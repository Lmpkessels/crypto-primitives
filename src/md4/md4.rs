use crate::md4::{ f, g, h };
use crate::utils::{ rotl, z };

/// MD4: Message Digestion function.
///
/// # Parameters
/// `m`: A vector of 512-bit message blocks, where each block is represented
///      as an array of sixteen 32-bit words in little-endian order.
///
/// # Description
/// - The algorithm performs three rounds of nonlinear functions, message word
///   reordering, bit rotations, and modular additions.  
/// - Each 512-bit message block updates the internal state `(A, B, C, D)`
///   to produce a 128-bit digest.
///
/// # Returns
/// A 128-bit digested hash key represented as an array of four 32-bit words.
///
/// # Reference
/// Based on the MD4 RFC-1320 specification:
/// [RFC-1320](https://datatracker.ietf.org/doc/html/rfc1320)
fn md4(m: &[[u32; 16]]) -> [u32; 4] {
    // Four word buffer for message digestion.
    // Order: A, B, C, D.
    let mut four_word_bffr: [u32; 4] = [
        0x67452301, // 0 = A
        0xefcdab89, // 1 = B
        0x98badcfe, // 2 = C
        0x10325476, // 3 = D
    ];

    // Index message order for 3 round.
    let message_order = [
        [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
        ],
        [
            0, 4, 8, 12, 1, 5, 9, 13, 2, 6, 10, 14, 3, 7, 11, 15
        ],
        [
            0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15
        ],
    ];
    
    // Rotl order for 3 rounds.
    let round_rotation = [
        [
            3, 7, 11, 19, 3, 7, 11, 19, 3, 7, 11, 19, 3, 7, 11, 19 
        ],
        [
            3, 5, 9, 13, 3, 5, 9, 13, 3, 5, 9, 13, 3, 5, 9, 13
        ],
        [
            3, 9, 11, 15, 3, 9, 11, 15, 3, 9, 11, 15, 3, 9, 11, 15
        ],    
    ];
            
    let mut digest = [0u32; 4];

    for x in m {
        // Buffer: (A, B, C, D) safed as lower: (a, b, c, d) instead of:
        // (AA, BB, CC, DD).
        let mut a = four_word_bffr[0];
        let mut b = four_word_bffr[1];
        let mut c = four_word_bffr[2];
        let mut d = four_word_bffr[3];

        for round in 0..3 {
            for j in 0..16 {
                let (func, msg, cnst, shft) = match round {
                    0 => (
                        // Round 1: nonlinear function F and no constant.
                        f(b, c, d), 
                        x[message_order[round][j]], 
                        0x00000000, 
                        round_rotation[round][j]),
                    1 => (
                        // Round 2: nonlinear function G with 0x5A827999.
                        g(b, c, d), x[message_order[round][j]], 
                        0x5A827999, 
                        round_rotation[round][j]),
                    2 => (
                        // Round 3: nonlinear function H with 0x6ED9EBA1.
                        h(b, c, d), 
                        x[message_order[round][j]], 
                        0x6ED9EBA1, 
                        round_rotation[round][j]),
                    _ => unreachable!()

                };

                let temp = a;
                a = d;
                d = c;
                c = b;
                b = rotl(z(z(z(temp, func), msg), cnst), shft);
            }

        }

        // Digest message.
        four_word_bffr[0] = z(four_word_bffr[0], a);
        four_word_bffr[1] = z(four_word_bffr[1], b);
        four_word_bffr[2] = z(four_word_bffr[2], c);
        four_word_bffr[3] = z(four_word_bffr[3], d);

        // Order digested message.
        digest = [
            four_word_bffr[0].swap_bytes(), 
            four_word_bffr[1].swap_bytes(), 
            four_word_bffr[2].swap_bytes(), 
            four_word_bffr[3].swap_bytes()
        ];
    }
    digest
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::padd_pars::{
        little_endian_padd, little_endian_pars
    };

    #[test]
    fn md4_compute_empty_string() {
        let msg = b"";
        let padded = little_endian_padd(msg); 
        let parsed = little_endian_pars(padded);
        let result = md4(&parsed);

        let expected = [
            0x31d6cfe0, 0xd16ae931, 0xb73c59d7, 0xe0c089c0
        ];

        assert_eq!((result), (expected));
    }

    #[test]
    fn md4_compute_a() {
        let msg = b"a";
        let padded = little_endian_padd(msg);
        let parsed = little_endian_pars(padded);
        let result = md4(&parsed);

        let expected = [
            0xbde52cb3, 0x1de33e46, 0x245e05fb, 0xdbd6fb24
        ];

        assert_eq!((result), (expected));
    }

    #[test]
    fn md4_compute_abc() {
        let msg = b"abc";
        let padded = little_endian_padd(msg); 
        let parsed = little_endian_pars(padded);
        let result = md4(&parsed);

        let expected = [
            0xa448017a, 0xaf21d852, 0x5fc10ae8, 0x7aa6729d
        ];

        assert_eq!((result), (expected));
    }

    #[test]
    fn md4_compute_long_abcde_string() {
        let msg = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcde\
                    fghijklmnopqrstuvwxyz0123456789";
        let padded = little_endian_padd(msg);
        let parsed = little_endian_pars(padded);
        let result = md4(&parsed);

        let expected = [
            0x043f8582, 0xf241db35, 0x1ce627e1, 0x53e7f0e4
        ];

        assert_eq!((result), (expected));
    }

    #[test]
    fn md4_compute_long_numbers_string() {
        let msg = b"1234567890123456789012345678901234567890\
                    1234567890123456789012345678901234567890";
        let padded = little_endian_padd(msg);
        let parsed = little_endian_pars(padded);
        let result = md4(&parsed);

        let expected = [
            0xe33b4ddc, 0x9c38f219, 0x9c3e7b16, 0x4fcc0536
        ];

        assert_eq!((result), (expected));
    }
}