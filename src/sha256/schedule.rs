use crate::utils::{z, small_sigma1, small_sigma0};

/// SHA256 Schedule function.
///
/// # Arguments:
/// Takes blocks as Vec<[u32; 16]>.
/// Then takes each 512-bit block outside of the blocks vector.
///
/// # Description
/// - Copies each block into m[0..16].
/// - Expands words 16..63 using small_sigma0 and small_sigma1 per SHA-256 spec.
/// - Produces a full 64-word schedule for each block.
///
/// # Returns
/// Scheduled message as vector [u32; 64] for downstream compression.
pub fn sched(blocks: Vec<[u32; 16]>) -> Vec<[u32; 64]> {
    let mut schedule: Vec<[u32; 64]> = Vec::new();

    for block in blocks {
        let mut m = [0u32; 64];
        
        for t in 0..16 {
            m[t] = block[t];
        }
        
        for t in 16..64 {
            m[t] = z(
                z(small_sigma1(m[t-2]), m[t-7]), 
                z(small_sigma0(m[t-15]), m[t-16])
            );
        }
        schedule.push(m);
    }
    schedule
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::padd_pars::{big_endian_padd, big_endian_pars};

    #[test]
    fn test_schedule_expansion_for_abc_known_good() {
        let msg = b"abc";
        let padding = big_endian_padd(msg);
        let parsing = big_endian_pars(padding);
        let result = sched(parsing);
        let expected_schedule: [u32; 64] = [
            0x61626380, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000018,
            0x61626380, 0x000f0000, 0x7da86405, 0x600003c6,
            0x3e9d7b78, 0x0183fc00, 0x12dcbfdb, 0xe2e2c38e,
            0xc8215c1a, 0xb73679a2, 0xe5bc3909, 0x32663c5b,
            0x9d209d67, 0xec8726cb, 0x702138a4, 0xd3b7973b,
            0x93f5997f, 0x3b68ba73, 0xaff4ffc1, 0xf10a5c62,
            0x0a8b3996, 0x72af830a, 0x9409e33e, 0x24641522,
            0x9f47bf94, 0xf0a64f5a, 0x3e246a79, 0x27333ba3,
            0x0c4763f2, 0x840abf27, 0x7a290d5d, 0x065c43da,
            0xfb3e89cb, 0xcc7617db, 0xb9e66c34, 0xa9993667,
            0x84badedd, 0xc21462bc, 0x1487472c, 0xb20f7a99,
            0xef57b9cd, 0xebe6b238, 0x9fe3095e, 0x78bc8d4b,
            0xa43fcf15, 0x668b2ff8, 0xeeaba2cc, 0x12b1edeb,
        ];


        let expected: Vec<[u32; 64]> = vec![expected_schedule];

        assert_eq!((result), (expected));
    }

    #[test]
    fn test_schedule_expansion_for_abc() {
        let msg = b"abc";
        let padding = big_endian_padd(msg);
        let parsing = big_endian_pars(padding); 
        let result = sched(parsing.clone());
        let mut expected: Vec<[u32; 64]> = Vec::new();
        
        for block in parsing {
            let mut m = [0u32; 64];

            for t in 0..16 {
                m[t] = block[t];
            }
        
            for t in 16..64 {
                m[t] = z(
                    z(small_sigma1(m[t-2]), m[t-7]), 
                    z(small_sigma0(m[t-15]), m[t-16])
                );
            } 
            expected.push(m);
        }

        assert_eq!((result), (expected));
    }
}