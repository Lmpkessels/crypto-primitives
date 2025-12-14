// Tata: XORs each bit in the state with the parities of two columns in the 
// array.
pub fn theta_func(a: &[[u64; 5]; 5]) -> [[u64; 5]; 5] {
    let mut c: [u64; 5] = [0; 5];
    let mut d: [u64; 5] = [0; 5];
    let mut a_: [[u64; 5]; 5] = [[0; 5]; 5];

    // 1. Column parities
    for x in 0..5 {
        c[x] = a[x][0] ^ a[x][1] ^ a[x][2] ^ a[x][3] ^ a[x][4];
    }

    // 2. D[x] from neighbors
    for x in 0..5 {
        d[x] = c[(x + 4) % 5] ^ c[(x + 1) % 5].rotate_left(1);
    }

    // 3. Apply to state
    for x in 0..5 {
        for y in 0..5 {
            a_[x][y] = a[x][y] ^ d[x];
        }
    }

    a_
}