// Rho: to ofset each lane.
pub fn rho_func(a: &[[u64; 5]; 5]) -> [[u64; 5]; 5] {
    let mut a_: [[u64; 5]; 5] = [[0; 5]; 5];
    a_[0][0] = a[0][0]; 

    let mut x = 1;
    let mut y = 0;

    for t in 0..24 {
        let r = ((t + 1) * (t + 2)) / 2;
        a_[x][y] = a[x][y].rotate_left(r as u32);

        let new_x = y;
        let new_y = (2 * x + 3 * y) % 5;
        x = new_x;
        y = new_y;
    }

    a_
}