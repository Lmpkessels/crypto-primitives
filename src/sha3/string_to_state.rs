// String to state: convert string (lanes) to array (state).
pub fn string_to_state(lanes: &[u64; 25]) -> [[u64; 5]; 5] {
    let mut a = [[0u64; 5]; 5];

    for x in 0..5 {
        for y in 0..5 {
            a[x][y] = lanes[x + 5 * y];
        }
    }

    a
}