use crate::utils::rotl;

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