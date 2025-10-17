/// Padding message in little endian order.
pub fn little_endian_padd(msg: &[u8]) -> Vec<u8> {
    let mut bytes: Vec<u8> = msg.to_vec();

    // Append 1 for end of message.
    // And all 0s to fill the block size.
    bytes.push(0x80);
    while bytes.len() % 64 != 56 {
        bytes.push(0x00);
    }

    // Work with byte range.
    let msg_len_bits = (msg.len() as u64) * 8;
    for i in 0..8 {
        bytes.push((msg_len_bits >> (i * 8)) as u8); // little-endian
    }

    bytes
}

/// Little-endian parsing stores the least significant byte (LSB).
/// at the lowest memory address,
/// and the most significant byte (MSB) at the highest memory address.
pub fn little_endian_pars(bytes: Vec<u8>) -> Vec<[u32; 16]> {
    let mut words: Vec<u32> = Vec::new();
    let mut j = 0;
    while j < bytes.len() {
        let b0 = bytes[j] as u32;
        let b1 = bytes[j + 1] as u32;
        let b2 = bytes[j + 2] as u32;
        let b3 = bytes[j + 3] as u32;

        let word = (b0) | (b1 << 8) | (b2 << 16) | (b3 << 24);

        words.push(word);

        j += 4;
    } 

    let mut blocks: Vec<[u32; 16]> = Vec::new();
    let mut k = 0;
    while k < words.len() {
        let mut block = [0u32; 16];
        let mut l = 0;
        while l < 16 {
            block[l] = words[k + l];
            
            l += 1
        }
        blocks.push(block); 
        k += 16;
    }

    blocks
}

/// Modular addition (x + y = (mod n))
///
/// Overflow is thrown away by the cast down to u32.
pub fn z(x: u32, y: u32) -> u32 {
    x.wrapping_add(y)
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

// Four auxiliary function working on 32 bit words.
pub fn f(x: u32, y: u32, z: u32) -> u32 { (x & y) | (!(x) & z) }
pub fn g(x: u32, y: u32, z: u32) -> u32 { (x & z) | (y & !(z)) }
pub fn h(x: u32, y: u32, z: u32) -> u32 { x ^ y ^ z }
pub fn i(x: u32, y: u32, z: u32) -> u32 { y ^ (x | !(z)) }

// Message digestion function with 4 working rounds.
fn md5(m: &[[u32; 16]]) -> [u32; 4] {
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

fn main() {
    let msg = b"abc";
    let padded = little_endian_padd(msg);
    let parsed = little_endian_pars(padded);
    let hash = md5(&parsed);

    println!("{:0x?}", hash);
}