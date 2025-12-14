// Lanes to state: to go from lanes (25 times a 64-bit word), to lanes (200 
// times a byte).
pub fn lanes_to_state(lanes: &[u64; 25]) -> [u8; 200] {
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