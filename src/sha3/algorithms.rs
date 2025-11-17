// String to state: convert string (lanes) to array (state).
fn string_to_state(lanes: &[u64; 25]) -> [[u64; 5]; 5] {
    let mut a = [[0u64; 5]; 5];

    for x in 0..5 {
        for y in 0..5 {
            a[x][y] = lanes[x + 5 * y];
        }
    }

    a
}

// State to string: convert array (state) to string (lanes).
fn state_to_string(a: &[[u64; 5]; 5]) -> [u64; 25] {
    let mut lanes = [0u64; 25];

    for x in 0..5 {
        for y in 0..5 {
            lanes[x + 5 * y] = a[x][y];
        }
    }

    lanes
}

// Tata: XORs each bit in the state with the parities of two columns in the 
// array.
fn theta_func(a: &[[u64; 5]; 5]) -> [[u64; 5]; 5] {
    let mut c: [u64; 5] = [0; 5];
    let mut d: [u64; 5] = [0; 5];
    let mut a_: [[u64; 5]; 5] = [[0; 5]; 5];

    // 1. Column parities
    for x in 0..5 {
        c[x] = a[x][0] ^ a[x][1] ^ a[x][2] ^ a[x][3] ^ a[x][4];
    }

    // 2. D[x] from neighbors
    for x in 0..5 {
        d[x] = c[(x + 4) % 5] ^ c[(x + 1) % 5].rotate_left(1);
    }

    // 3. Apply to state
    for x in 0..5 {
        for y in 0..5 {
            a_[x][y] = a[x][y] ^ d[x];
        }
    }

    a_
}

// Rho: to ofset each lane.
fn rho_func(a: &[[u64; 5]; 5]) -> [[u64; 5]; 5] {
    let mut a_: [[u64; 5]; 5] = [[0; 5]; 5];
    a_[0][0] = a[0][0]; 

    let mut x = 1;
    let mut y = 0;

    for t in 0..24 {
        let r = ((t + 1) * (t + 2)) / 2;
        a_[x][y] = a[x][y].rotate_left(r as u32);

        let new_x = y;
        let new_y = (2 * x + 3 * y) % 5;
        x = new_x;
        y = new_y;
    }

    a_
}

// Pi: rearrange positions of lanes.
fn pi_func(a: &[[u64; 5]; 5]) -> [[u64; 5]; 5] {
    let mut out = [[0u64; 5]; 5];

    for x in 0..5 {
        for y in 0..5 {
            let new_x = y;
            let new_y = (2 * x + 3 * y) % 5;
            out[new_x][new_y] = a[x][y];
        }
    }

    out
}

// Chi: XORs bit-by-bit with non-linear function of two other bits in its row.
fn chi_func(a: &[[u64; 5]; 5]) -> [[u64; 5]; 5] {
    let mut a_: [[u64; 5]; 5] = [[0; 5]; 5];

    for x in 0..5 {
        for y in 0..5 {
            a_[x][y] = a[x][y] ^ ((!a[(x + 1) % 5][y]) & (a[(x + 2) % 5][y])); 
        }
    }

    a_
}

// Rc: inject unique bit in each round to break symmetry between rounds.
fn rc_func(t: u64) -> u8 {
    if t % 255 == 0 {
        return 1;
    };

    let mut r: [u8; 8] = [1, 0, 0, 0, 0, 0, 0, 0];

    let mut i = 0;
    while i < t % 255 {
        
        let mut nine: [u8; 9] = [0u8; 9];
        nine[0] = 0;
        
        let mut j = 0;
        while j < 8 {
            nine[j + 1] = r[j];
            j += 1;
        }

        let fb = nine[8];

        nine[0] = nine[0] ^ fb;
        nine[4] = nine[4] ^ fb;
        nine[5] = nine[5] ^ fb;
        nine[6] = nine[6] ^ fb;

        let mut k = 0;
        while k < 8 {
            r[k] = nine[k];
            k += 1; 
        }

        i += 1;
    }

    r[0]
}

// Lota: to modify the bits of Lane(0, 0) in a manner depending on the round
// index Ir.
fn iota_func(a: &[[u64; 5]; 5], ir: usize) -> [[u64; 5]; 5] {
    let mut a_: [[u64; 5]; 5] = *a;
    let mut rc = 0u64;

    for j in 0..7 {
        if rc_func(j as u64 + 7 * ir as u64) != 0 {
            rc ^= 1u64 << ((1 << j) -1);
        }
    }

    a_[0][0] ^= rc;

    a_
}

// RnD: for applying stepmapping to receive a transformed state.
fn rnd_func(a: &[[u64; 5]; 5], ir: usize) -> [[u64; 5]; 5] {
    let tata = theta_func(a);
    let rho = rho_func(&tata);
    let pi = pi_func(&rho);
    let shi = chi_func(&pi);
    let lota = iota_func(&shi, ir);

    lota
}

// Keccak permution: applied for the given number of rounds.
fn keccak_p(lanes: &[u64; 25], rounds: usize) -> [u64; 25] {
    let mut a = string_to_state(lanes);

    for ir in 0..rounds {
        a = rnd_func(&a, ir);
    }

    state_to_string(&a)
}

// Keccak padding: to produce output string of decired length.
fn keccak_padd(mut v: Vec<u8>, rate: usize) -> Vec<u8> {
    let rate_bytes = rate / 8;

    // Domain sepperator.
    v.push(0x06);

    // Pad till end of block with all 0s.
    while (v.len() % rate_bytes) != rate_bytes - 1 {
        v.push(0x00);
    }

    // Final byte, end of message.
    v.push(0x80);

    v
}

// State to lanes: go from state (200 times a byte) to lanes (25 times a 64-bit 
// word).
fn state_to_lanes(state: &[u8; 200]) -> [u64; 25] {
    let mut lanes = [0u64; 25];

    for x in 0..5 {
        for y in 0..5 {
            let idx = 5 * y + x;
            let mut lane = 0u64;

            for j in 0..8 {
                lane |= (state[idx * 8 + j] as u64) << (8*j);
            }

            lanes[idx] = lane;
        }
    }

    lanes
}

// Lanes to state: to go from lanes (25 times a 64-bit word), to lanes (200 
// times a byte).
fn lanes_to_state(lanes: &[u64; 25]) -> [u8; 200] {
    let mut state = [0u8; 200];

    for x in 0..5 {
        for y in 0..5 {
            let idx = 5 * y + x;
            let lane = lanes[idx];

            for j in 0..8 {
                state[idx * 8 + j] = ((lane >> (8 * j)) & 0xFF) as u8;
            }
        }
    }

    state
}

// Sponge construction: to absorb an arbitrary number of input bits into the
// state function, to afterwards squeeze an arbitrary number of output bits.
fn sponge(msg: &[u8]) -> Vec<u8> {
    let rate_in_bits = 1088;
    let rate_in_bytes = rate_in_bits / 8;
    let output_length = 32;
    
    let mut state = [0u8; 200];

    let mut padded = keccak_padd(msg.to_vec(), rate_in_bits);

    let mut i = 0;

    // Absorb part.
    while i < padded.len() {
        let mut j = 0;

        while j < rate_in_bytes && (i + j) < padded.len() {
            state[j] ^= padded[i + j];
            j += 1;
        }

        let lanes = state_to_lanes(&state);
        let new_lanes = keccak_p(&lanes, 24);
        state = lanes_to_state(&new_lanes);

        i += rate_in_bytes;
    }

    let mut new_msg_string: Vec<u8> = Vec::new();

    // Squeeze part.
    while new_msg_string.len() < output_length {
        let mut j = 0;
        while j < rate_in_bytes && new_msg_string.len() < output_length {
            new_msg_string.push(state[j]);
            j += 1;
        }

        // Truncate.
        if new_msg_string.len() >= output_length {
            break;
        };
    
        let lanes = state_to_lanes(&state);
        let new_lanes = keccak_p(&lanes, 24);
        state = lanes_to_state(&new_lanes);

    }
    new_msg_string
}

// Bytes to hex: goes from message received in bytes to hexdecimal output.
fn bytes_to_hex(bytes: &[u8]) -> Vec<u8> {
    let hex_table = b"0123456789abcdef";
    let mut out = Vec::with_capacity(bytes.len() * 2);

    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];

        // High nibble
        let hi = (b >> 4) & 0x0f;
        out.push(hex_table[hi as usize]);

        // Low nibble
        let lo = b & 0x0f;
        out.push(hex_table[lo as usize]);

        i += 1;
    }

    out
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sha3_abc() {
        let out = sponge(b"abc");
        let hex = bytes_to_hex(&out);
        assert_eq!(
            hex,
            b"3a985da74fe225b2045c172d6bd390bd\
              855f086e3e9d525b46bfe24511431532"
        );
    }

    #[test]
    fn test_empty() {
        let out = sponge(b"");
        let hex = bytes_to_hex(&out);
        assert_eq!(
            hex,
            b"a7ffc6f8bf1ed76651c14756a061d662\
              f580ff4de43b49fa82d80a4b80f8434a"
        );
    }

    #[test]
    fn test_one_million_a() {
        let msg = vec![b'a'; 1_000_000];
        let out = sponge(&msg);
        let hex = bytes_to_hex(&out);
        assert_eq!(
            hex,
            b"5c8875ae474a3634ba4fd55ec85bffd6\
              61f32aca75c6d699d0cdcb6c115891c1"
        );
    }

}
