// State to lanes: go from state (200 times a byte) to lanes (25 times a 64-bit 
// word).
pub fn state_to_lanes(state: &[u8; 200]) -> [u64; 25] {
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