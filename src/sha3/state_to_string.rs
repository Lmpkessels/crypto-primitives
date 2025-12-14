// State to string: convert array (state) to string (lanes).
pub fn state_to_string(a: &[[u64; 5]; 5]) -> [u64; 25] {
    let mut lanes = [0u64; 25];

    for x in 0..5 {
        for y in 0..5 {
            lanes[x + 5 * y] = a[x][y];
        }
    }

    lanes
}