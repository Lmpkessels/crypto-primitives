// Chi: XORs bit-by-bit with non-linear function of two other bits in its row.
pub fn chi_func(a: &[[u64; 5]; 5]) -> [[u64; 5]; 5] {
    let mut a_: [[u64; 5]; 5] = [[0; 5]; 5];

    for x in 0..5 {
        for y in 0..5 {
            a_[x][y] = a[x][y] ^ ((!a[(x + 1) % 5][y]) & (a[(x + 2) % 5][y])); 
        }
    }

    a_
}