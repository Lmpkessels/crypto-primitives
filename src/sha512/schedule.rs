use crate::utils64::{
    small_sigma0_64, small_sigma1_64
};

pub fn schedule(msg_as_blocks: &[[u64; 16]]) -> Vec<[u64; 80]> {
    let mut w: Vec<[u64; 80]> = Vec::new();
    let mut m = [0u64; 80];

    for block in msg_as_blocks {
        for i in 0..16 {
            m[i] = block[i];
        }

        for t in 16..80 {
            m[t] = small_sigma1_64(m[t-2])
                        .wrapping_add(m[t-7])
                        .wrapping_add(small_sigma0_64(m[t-15])
                        .wrapping_add(m[t-16]))
            ;
        }

        w.push(m);
    }

    w
}