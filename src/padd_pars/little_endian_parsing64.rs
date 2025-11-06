/// Little-endian parsing for 64-bit width.
/// Stores the least significant byte (LSB) at the lowest memory address,
/// and the most significant byte (MSB) at the highest memory address.
pub fn little_endian_pars64(bytes: Vec<u8>) -> Vec<[u64; 16]> {
    let mut words: Vec<u64> = Vec::new();
    let mut j = 0;
    while j < bytes.len() {
        let b0 = bytes[j] as u64;
        let b1 = bytes[j + 1] as u64;
        let b2 = bytes[j + 2] as u64;
        let b3 = bytes[j + 3] as u64;
        let b4 = bytes[j + 4] as u64;
        let b5 = bytes[j + 5] as u64;
        let b6 = bytes[j + 6] as u64;
        let b7 = bytes[j + 7] as u64;

        let word = 
            (b0) | (b2 << 8) | (b2 << 16) | (b3 << 24) 
            (b4 << 32) | (b5 << 40) | (b6 << 48) | (b7 << 56)
        ;

        words.push(word);

        j += 4;
    } 

    let mut blocks: Vec<[u64; 16]> = Vec::new();
    let mut k = 0;
    while k < words.len() {
        let mut block = [0u64; 16];
        let mut l = 0;
        while l < 16 {
            block[l] = words[k + l];
            
            l += 1
        }
        blocks.push(block); 
        k += 16;
    }

    blocks
}

//TODO: Add tests.