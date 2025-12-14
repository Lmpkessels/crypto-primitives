// Pi: rearrange positions of lanes.
pub fn pi_func(a: &[[u64; 5]; 5]) -> [[u64; 5]; 5] {
    let mut out = [[0u64; 5]; 5];

    for x in 0..5 {
        for y in 0..5 {
            let new_x = y;
            let new_y = (2 * x + 3 * y) % 5;
            out[new_x][new_y] = a[x][y];
        }
    }

    out
}