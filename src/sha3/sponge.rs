use crate::padd_pars::keccak_padding::keccak_padd;
use crate::sha3::{
    state_to_lanes, keccak_permutation, lanes_to_state
};

// Sponge construction: to absorb an arbitrary number of input bits into the
// state function, to afterwards squeeze an arbitrary number of output bits.
pub fn sponge(msg: &[u8]) -> Vec<u8> {
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
        let new_lanes = keccak_permutation(&lanes, 24);
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
        let new_lanes = keccak_permutation(&lanes, 24);
        state = lanes_to_state(&new_lanes);

    }
    new_msg_string
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sha3::bytes_to_hex;

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