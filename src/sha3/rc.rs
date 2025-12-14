// Rc: inject unique bit in each round to break symmetry between rounds.
pub fn rc_func(t: u64) -> u8 {
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