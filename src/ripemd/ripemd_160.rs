fn z(x: u32, y: u32) -> u32 {
    x.wrapping_add(y)
}

/// Rotate left (ROTL).
///
/// Rotate x left by n bits within a 32-bit word (wraps bits around).
fn rotl(x: u32, n: u32) -> u32 {
    // Normalize 0..31.
    let n = n & 31;
    // Use the complement count within the 32-bit word.
    (x << n) | (x >> (32 - n))
}

fn padd(msg: &[u8]) -> Vec<u8> {
    let mut bytes = msg.to_vec();
    
    // Append the 0x80 byte.
    bytes.push(0x80);
    
    // Pad with zeros until length % 64 == 56.
    while bytes.len() % 64 != 56 {
        bytes.push(0x00);
    }
    
    // Append message length in bits, little-endian.
    let msg_bits = (msg.len() as u64) * 8;
    for i in 0..8 {
        bytes.push(((msg_bits >> (8 * i)) & 0xFF) as u8);
    }
    
    bytes
}

fn small_endian_pars(bytes: Vec<u8>) -> Vec<[u32; 16]> {
    let mut words: Vec<u32> = Vec::new();
    let mut j = 0;
    while j < bytes.len() {
        // Pars message length into 32-bit words.
        let b0 = bytes[j] as u32;
        let b1 = bytes[j + 1] as u32;
        let b2 = bytes[j + 2] as u32;
        let b3 = bytes[j + 3] as u32;

        let word = (b0) | (b1 << 8) | (b2 << 16) | (b3 << 24);

        words.push(word);

        // Increment j by 4 since 8 * 4 = 32.
        j += 4;
    } 

    let mut blocks: Vec<[u32; 16]> = Vec::new();
    let mut k = 0;
    while k < words.len() {
        let mut block = [0u32; 16];
        let mut l = 0;
        while l < 16 {
            // Store 16 32-bit words in block such that
            // block == 512 bits.
            block[l] = words[k + l];
            
            l += 1
        }
        blocks.push(block); 
        // Increment by 16 to stay in correct
        // word range.
        k += 16;
    }

    blocks
}

fn f1(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

fn f2(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (!(x) & z)
}

fn f3(x: u32, y: u32, z: u32) -> u32 {
    (x | !(y)) ^ z
}

fn f4(x: u32, y: u32, z: u32) -> u32 {
    (x & z) | (y & !(z))
}

fn f5(x: u32, y: u32, z: u32) -> u32 {
    x ^ (y | !(z))
}

// fn 1-5 for left round.
fn f_left(j: u32, x: u32, y: u32, z: u32) -> u32 {
    match j {
        0..=15 => f1(x, y, z),
        16..=31 => f2(x, y, z),
        32..=47 => f3(x, y, z),
        48..=63 => f4(x, y, z),
        64..=79 => f5(x, y, z),
        _ => unreachable!(),
    }
}

// fn 5-1 for right round.
fn f_right(j: u32, x: u32, y: u32, z: u32) -> u32 {
    match j {
        0..=15 => f5(x, y, z),
        16..=31 => f4(x, y, z),
        32..=47 => f3(x, y, z),
        48..=63 => f2(x, y, z),
        64..=79 => f1(x, y, z),
        _ => unreachable!(),
    }
}

// Round constants for left round.
fn l_round_constants(j: u32) -> u32 {
    match j {
        0..=15 => 0x00000000,
        16..=31 => 0x5a827999,
        32..=47 => 0x6ed9eba1,
        48..=63 => 0x8f1bbcdc,
        64..=79 => 0xa953fd4e,
        _ => unreachable!(),
    }
}

// Round constants for right round.
fn r_round_constants(j: u32) -> u32 {
    match j {
        0..=15 => 0x50a28be6,
        16..=31 => 0x5c4dd124,
        32..=47 => 0x6d703ef3,
        48..=63 => 0x7a6d76e9,
        64..=79 => 0x00000000,
        _ => unreachable!(),
    }
}

fn ripemd(m: Vec<[u32; 16]>) -> [u32; 5] {
    // Left 5 round message word selection.
    let left_r: [u32; 80] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
        7, 4, 13, 1, 10, 6, 15, 3, 12, 0, 9, 5, 2, 14, 11, 8,
        3, 10, 14, 4, 9, 15, 8, 1, 2, 7, 0, 6, 13, 11, 5, 12,
        1, 9, 11, 10, 0, 8, 12, 4, 13, 3, 7, 15, 14, 5, 6, 2,
        4, 0, 5, 9, 7, 12, 2, 10, 14, 1, 3, 8, 11, 6, 15, 13,
    ];

    // Right 5 round message word selection.
    let right_r: [u32; 80] = [
        5, 14, 7, 0, 9, 2, 11, 4, 13, 6, 15, 8, 1, 10, 3, 12,
        6, 11, 3, 7, 0, 13, 5, 10, 14, 15, 8, 12, 4, 9, 1, 2,
        15, 5, 1, 3, 7, 14, 6, 9, 11, 8, 12, 2, 10, 0, 4, 13,
        8, 6, 4, 1, 3, 11, 15, 0, 5, 12, 2, 13, 9, 7, 10, 14,
        12, 15, 10, 4, 1, 5, 8, 7, 6, 2, 13, 14, 0, 3, 9, 11
    ];

    // Left 5 round shifts.
    let left_s: [u32; 80] = [
        11, 14, 15, 12, 5, 8, 7, 9, 11, 13, 14, 15, 6, 7, 9, 8,
        7, 6, 8, 13, 11, 9, 7, 15, 7, 12, 15, 9, 11, 7, 13, 12,
        11, 13, 6, 7, 14, 9, 13, 15, 14, 8, 13, 6, 5, 12, 7, 5,
        11, 12, 14, 15, 14, 15, 9, 8, 9, 14, 5, 6, 8, 6, 5, 12,
        9, 15, 5, 11, 6, 8, 13, 12, 5, 12, 13, 14, 11, 8, 5, 6,
    ];

    // Right 5 round shifts.
    let right_s: [u32; 80] = [
        8, 9, 9, 11, 13, 15, 15, 5, 7, 7, 8, 11, 14, 14, 12, 6,
        9, 13, 15, 7, 12, 8, 9, 11, 7, 7, 12, 7, 6, 15, 13, 11,
        9, 7, 15, 11, 8, 6, 6, 14, 12, 13, 5, 14, 13, 13, 7, 5,
        15, 5, 8, 11, 14, 14, 6, 14, 6, 9, 12, 9, 12, 5, 15, 8,
        8, 5, 12, 9, 12, 5, 14, 6, 8, 13, 6, 5, 15, 13, 11, 11
    ];
    
    // Initial hexdecimal values.
    let mut h0 = 0x67452301;
    let mut h1 = 0xefcdab89;
    let mut h2 = 0x98badcfe; 
    let mut h3 = 0x10325476;
    let mut h4 = 0xc3d2e1f0;
    let mut digest = [0u32; 5];
    
    //TODO: Look if the occuring bug is found at updating
    //TODO: value h0-h5, a-e, and a'-e'. 
    // Get blocks one-by-one.
    for x in &m {
        // Left and right hexdecimal values.
        let mut left_a = h0;
        let mut left_b = h1;
        let mut left_c = h2;
        let mut left_d = h3;
        let mut left_e = h4;
        let mut right_a = h0;
        let mut right_b = h1;
        let mut right_c = h2;
        let mut right_d = h3;
        let mut right_e = h4;

        // Right and left temporay values.
        let mut left_temp;
        let mut right_temp;

        // Start of message digest.
        for j in 0..80 {
            left_temp = z(
                rotl(
                    z(
                        z(
                            z(
                                //TODO: Look here if value is updated.
                                left_a, f_left(j, left_b, left_c, left_d)), 
                            x[left_r[j as usize] as usize]), 
                        l_round_constants(j)), 
                    left_s[j as usize]),
                left_e
            );

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
                                //TODO: Look here if value is updated.
                                right_a, f_right(j, right_b, right_c, right_d)), 
                            x[right_r[j as usize] as usize]), 
                        r_round_constants(j)), 
                    right_s[j as usize]),
                right_e, 
            );
            
            right_a = right_e;
            right_e = right_d;
            right_d = rotl(right_c, 10);
            right_c = right_b;
            right_b = right_temp;
            
        }
        let t = h1.wrapping_add(left_c).wrapping_add(right_d);
        let new_h1 = h2.wrapping_add(left_d).wrapping_add(right_e);
        let new_h2 = h3.wrapping_add(left_e).wrapping_add(right_a);
        let new_h3 = h4.wrapping_add(left_a).wrapping_add(right_b);
        let new_h4 = h0.wrapping_add(left_b).wrapping_add(right_c);
        let new_h0 = t;
        h1 = new_h1;
        h2 = new_h2;
        h3 = new_h3;
        h4 = new_h4;
        
        digest = [h0, h1, h2, h3, h4];
    }
    digest
}

fn main() {
    let msg = b"";
    let padded = padd(msg);
    let parsed = small_endian_pars(padded);
    let test = ripemd(parsed);
    println!("{:0x?}", test);
}