use crate::utils::rotl;

/// SHA1 Schedule function.
///
/// # Arguments:
/// Takes blocks as Vec<[u32; 16]>.
/// Then takes each 512-bit block outside of the vector.
///
/// # Description
/// - Copies each 16 32-bit word block into m[0..16].
/// - Expands words 16..80 using ROTL.
/// - Produces a full 80 32-bit word schedule for each block.
///
/// # Returns
/// Scheduled message as vector [u32; 80] for downstream compression.
pub fn schedule(parsed: &[[u32; 16]]) -> Vec<[u32; 80]> {
    let mut scheduled: Vec<[u32; 80]> = Vec::new();

    for block in parsed {
        let mut m = [0u32; 80];

        for i in 0..16 {
            m[i] = block[i];
        };

        for t in 16..80 {
            m[t] = rotl(m[t-3] ^ m[t-8] ^ m[t-14] ^ m[t-16], 1);
        }

        scheduled.push(m);
    }
    scheduled

}