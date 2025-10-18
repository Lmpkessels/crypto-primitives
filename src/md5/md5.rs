use crate::md5::functions::{ f, g, h, i };
use crate::utils::{ rotl, z };

/// MD5: Message Digestion function.
///
/// # Parameters
/// `m`: A vector of 512-bit message blocks, where each block is represented
///      as an array of sixteen 32-bit words in little-endian order.
///
/// # Description
/// - The algorithm performs four rounds of nonlinear functions, message word
///   reordering, bit rotations, and modular additions.  
/// - Each 512-bit message block updates the internal state `(A, B, C, D)`
///   to produce a 128-bit digest.
///
/// # Returns
/// A 128-bit digested hash key represented as an array of four 32-bit words.
///
/// # Reference
/// Based on the MD5 RFC-1321 specification:
/// [RFC-1321](https://www.rfc-editor.org/rfc/pdfrfc/rfc1321.txt.pdf)
fn md5(m: &[[u32; 16]]) -> [u32; 4] {
    // Four word buffer for message digestion.
    // Order: A, B, C, D.
    let mut four_word_bffr: [u32; 4] = [
        0x67452301, // 0 = A
        0xefcdab89, // 1 = B
        0x98badcfe, // 2 = C
        0x10325476, // 3 = D
    ];

    let message_order = [
        // Round 1
        [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
        ],
        // Round 2
        [
            1, 6, 11, 0, 5, 10, 15, 4, 9, 14, 3, 8, 13, 2, 7, 12
        ],
        // Round 3
        [
            5, 8, 11, 14, 1, 4, 7, 10, 13, 0, 3, 6, 9, 12, 15, 2
        ],
        // Round 4
        [
            0, 7, 14, 5, 12, 3, 10, 1, 8, 15, 6, 13, 4, 11, 2, 9
        ],
    ];

    let mut round_rotation = [
        // Round 1
        [
            7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22
        ],
        // Round 2
        [
            5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20
        ],
        // Round 3
        [
            4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23 
        ],
        // Round 4
        [
            6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21
        ]
    ];

    let i_th_element = [
        // Round 1
        [
            0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
            0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
            0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
            0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821
        ],
        // Round 2
        [
            0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
            0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
            0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
            0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a
        ],
        // Round 3
        [
            0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
            0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
            0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
            0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665
        ],
        // Round 4
        [
            0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
            0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
            0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
            0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391
        ]
    ];


    let mut digest = [0u32; 4];

    for x in m {
        let mut a = four_word_bffr[0];
        let mut b = four_word_bffr[1];
        let mut c = four_word_bffr[2];
        let mut d = four_word_bffr[3];
        for round in 0..4 {
            for j in 0..16 {
                let (func, msg, i_th, shft) = match round {
                    // Round (1, 2, 3, 4): nonlinear function (F, G, H, I) 
                    // + message block (x) + the it-h element of the table 
                    // + the defined round rotations.
                    0 => (
                        f(b, c, d), 
                        x[message_order[round][j]], 
                        i_th_element[round][j], 
                        round_rotation[round][j]
                    ),
                    1 => (
                        g(b, c, d), 
                        x[message_order[round][j]], 
                        i_th_element[round][j], 
                        round_rotation[round][j]
                    ),
                    2 => (
                        h(b, c, d), 
                        x[message_order[round][j]], 
                        i_th_element[round][j], 
                        round_rotation[round][j]
                    ),
                    3 => (
                        i(b, c, d), 
                        x[message_order[round][j]], 
                        i_th_element[round][j], 
                        round_rotation[round][j]
                    ),
                    _ => unreachable!(),
                };

                let temp = d;
                d = c;
                c = b;
                b = z(b, rotl(z(z(z(a, func), msg), i_th), shft));
                a = temp;
            }
        }

        // Message digestion
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
mod new {
    use super::*;
    use crate::padd_pars::{
        little_endian_padd, little_endian_pars
    };


    #[test]
    fn md5_compute_empty_string() {
        let msg = b"";
        let padded = little_endian_padd(msg);
        let parsed = little_endian_pars(padded);
        let result = md5(&parsed);

        let expected = [
            0xd41d8cd9, 0x8f00b204, 0xe9800998, 0xecf8427e
        ];

        assert_eq!((expected), (result));
    }

    #[test]
    fn md5_compute_one_a() {
        let msg = b"a";
        let padded = little_endian_padd(msg);
        let parsed = little_endian_pars(padded);
        let result = md5(&parsed);

        let expected = [
            0x0cc175b9, 0xc0f1b6a8, 0x31c399e2, 0x69772661
        ];

        assert_eq!((expected), (result));
    }

    #[test]
    fn md5_compute_abc() {
        let msg = b"abc";
        let padded = little_endian_padd(msg);
        let parsed = little_endian_pars(padded);
        let result = md5(&parsed);

        let expected = [
            0x90015098, 0x3cd24fb0, 0xd6963f7d, 0x28e17f72
        ];

        assert_eq!((expected), (result));
    }

    #[test]
    fn md5_compute_expenditure_next_second_block() {
        let msg = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcde\
                    fghijklmnopqrstuvwxyz0123456789";
        let padded = little_endian_padd(msg);
        let parsed = little_endian_pars(padded);
        let result = md5(&parsed);

        let expected = [
            0xd174ab98, 0xd277d9f5, 0xa5611c2c, 0x9f419d9f
        ];

        assert_eq!((expected), (result));
    }

    #[test]
    fn md5_compute_expenditure_into_next_block_with_all_numbers() {
        let msg = b"123456789012345678901234567\
                    890123456789012345678901234\
                    56789012345678901234567890";
        let padded = little_endian_padd(msg);
        let parsed = little_endian_pars(padded);
        let result = md5(&parsed);

        let expected = [
            0x57edf4a2, 0x2be3c955, 0xac49da2e, 0x2107b67a
        ];

        assert_eq!((expected), (result));
    }
}