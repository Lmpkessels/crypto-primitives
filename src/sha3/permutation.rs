use crate::sha3::{
    string_to_state, rnd_func, state_to_string
};

// Keccak permution: applied for the given number of rounds.
pub fn keccak_permutation(lanes: &[u64; 25], rounds: usize) -> [u64; 25] {
    let mut a = string_to_state(lanes);

    for ir in 0..rounds {
        a = rnd_func(&a, ir);
    }

    state_to_string(&a)
}