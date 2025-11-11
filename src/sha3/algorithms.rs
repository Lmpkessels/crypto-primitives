// String to state: convert string (lanes) to array (state).
fn string_to_state(lanes: &[u64; 25]) -> [[u64; 5]; 5] {
    let mut a: [[u64; 5]; 5] = [[0; 5]; 5];
    let mut i = 0;

    for x in 0..5 {
        for y in 0..5 {
            a[x][y] = lanes[i];
            i += 1;
        }
    }

    a
}

// State to string: convert array (state) to string (lanes).
fn state_to_string(a: &[[u64; 5]; 5]) -> [u64; 25] {
    let mut lanes: [u64; 25] = [0; 25];
    let mut i = 0;

    for x in 0..5 {
        for y in 0..5 {
            lanes[i] = a[x][y];
            i += 1;
        }
    }

    lanes
}

// Tata: XORs each bit in the state with the parities of two columns in the 
// array.
fn tata_func(a: &[[u64; 5]; 5]) -> [[u64; 5]; 5] {
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

// Rho: to ofset each lane.
fn rho_func(a: &[[u64; 5]; 5]) -> [[u64; 5]; 5] {
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

// Pi: to rearange positions of lanes.
fn pi_func(a: &[[u64; 5]; 5]) -> [[u64; 5]; 5] {
    let mut a_: [[u64; 5]; 5] = [[0; 5]; 5];
    
    for x in 0..5 {
        for y in 0..5 {
            a_[x][y] = a[(x + 3 * y) % 5][x];
        }
    }

    a_
}

// Chi: XORs bit-by-bit with non-linear function of two other bits in its row.
fn chi_func(a: &[[u64; 5]; 5]) -> [[u64; 5]; 5] {
    let mut a_: [[u64; 5]; 5] = [[0; 5]; 5];

    for x in 0..5 {
        for y in 0..5 {
            a_[x][y] = a[x][y] ^ a[(x + 1) % 5][y] & a[(x + 2) % 5][y]; 
        }
    }

    a_
}

// Rc: inject unique bit in each round to break symmetry between rounds.
fn rc_func(t: u64) -> u8 {
    if t % 255 == 0 {
        return 1;
    };

    let mut r: [u8; 8] = [1, 0, 0, 0, 0, 0, 0, 0];

    let mut i = 0;
    while i < t % 255 {
        
        let mut nine: [u8; 9] = [0u8; 9];
        nine[0] = 0;
        
        let mut j = 0;
        while j < 8 {
            nine[j + 1] = r[j];
            j += 1;
        }

        let fb = nine[8];

        nine[0] = nine[0] ^ fb;
        nine[4] = nine[4] ^ fb;
        nine[5] = nine[5] ^ fb;
        nine[6] = nine[6] ^ fb;

        let mut k = 0;
        while k < 8 {
            r[k] = nine[k];
            k += 1; 
        }

        i += 1;
    }

    r[0]
}

// Lota: to modify the bits of Lane(0, 0) in a manner depending on the round
// index Ir.
fn lota_func(a: &[[u64; 5]; 5], ir: u64) -> [[u64; 5]; 5] {
    let mut a_: [[u64; 5]; 5] = *a;

    let mut rc: u64 = 0;
    for j in 0..7 {
        if rc_func(j + 7 * ir) != 0 {
            rc ^= 1u64 << ((1 << j) -1);
        }
    }

    a_[0][0] ^= rc;

    a_
}

// RnD: for applying stepmapping to receive a transformed state.
fn rnd_func(a: &[[u64; 5]; 5], ir: u64) -> [[u64; 5]; 5] {
    let tata = tata_func(a);
    let rho = rho_func(&tata);
    let pi = pi_func(&rho);
    let shi = chi_func(&pi);
    let lota = lota_func(&shi, ir);

    lota
}

fn main() {
    let string = [
        2323, 222, 254125, 9143, 19348914, 
        1934482, 1934, 132444, 1212, 21213,
        2323, 222, 254125, 9143, 19348914, 
        1934482, 1934, 132444, 1212, 21213,
        2323, 222, 254125, 9143, 19348914,
    ];

    let a = 
    [
        [12, 11, 13, 1, 49], 
        [12, 111, 1, 93, 1], 
        [71, 121, 9, 11, 1],
        [9, 00, 1, 1, 1],
        [12, 1, 48, 2, 21],
    ];

    let test_tata = tata_func(&a);
    let test_rho = rho_func(&a);
    let test_pi = pi_func(&a);
    let test_chi = chi_func(&a);
    let test_rc = rc_func(510);
    let test_lota = lota_func(&a, 3);
    let test_rnd = rnd_func(&a, 3);
    let str_to_state = string_to_state(&string);
    let state_to_str = state_to_string(&a);

    println!("{test_tata:?}\n");
    println!("{test_rho:?}\n");
    println!("{test_pi:?}\n");
    println!("{test_chi:?}\n");
    println!("{test_rc:?}\n");
    println!("{test_lota:?}\n");
    println!("{test_rnd:?}\n");
    println!("{str_to_state:?}\n");
    println!("{state_to_str:?}\n");
}