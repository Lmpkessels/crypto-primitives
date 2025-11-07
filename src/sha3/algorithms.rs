// Tata
fn tata(a: &[[u64; 5]; 5]) -> [[u64; 5]; 5] {
    let mut c: [u64; 5] = [0; 5];
    let mut d: [u64; 5] = [0; 5];
    let mut a_: [[u64; 5]; 5] = [[0; 5]; 5];
    
    for x in 0..5 {
        c[x] = a[x][0] ^ a[x][1] ^ a[x][2] ^ a[x][3] ^ a[x][4];
        d[x] = c[(x + 4) % 5] ^ c[(x + 1) % 5].rotate_left(1);

        for y in 0..5 {
            a_[x][y] = a[x][y] ^ d[x]
        }
    }
    
    a_
}

// Rho
fn rho(a: &[[u64; 5]; 5]) -> [[u64; 5]; 5] {
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

// Pi
fn pi(a: &[[u64; 5]; 5]) -> [[u64; 5]; 5] {
    let mut a_: [[u64; 5]; 5] = [[0; 5]; 5];
    
    for x in 0..5 {
        for y in 0..5 {
            a_[x][y] = a[(x + 3 * y) % 5][x];
        }
    }

    a_
}

fn main() {
    let a = 
    [
        [12, 11, 13, 1, 49], 
        [12, 111, 1, 93, 1], 
        [71, 121, 9, 11, 1],
        [9, 00, 1, 1, 1],
        [12, 1, 48, 2, 21],
    ];

    let test_tata = tata(&a);
    let test_rho = rho(&a);
    let test_pi = pi(&a);

    println!("{test_tata:?}\n");
    println!("{test_rho:?}\n");
    println!("{test_pi:?}\n");
}



// Chi

// Lota

// Keccak_f1600