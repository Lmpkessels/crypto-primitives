/// Little-endian parsing stores the least significant byte (LSB).
/// at the lowest memory address,
/// and the most significant byte (MSB) at the highest memory address.
pub fn little_endian_pars(bytes: Vec<u8>) -> Vec<[u32; 16]> {
    let mut words: Vec<u32> = Vec::new();
    let mut j = 0;
    while j < bytes.len() {
        let b0 = bytes[j] as u32;
        let b1 = bytes[j + 1] as u32;
        let b2 = bytes[j + 2] as u32;
        let b3 = bytes[j + 3] as u32;

        let word = (b0) | (b1 << 8) | (b2 << 16) | (b3 << 24);

        words.push(word);

        j += 4;
    } 

    let mut blocks: Vec<[u32; 16]> = Vec::new();
    let mut k = 0;
    while k < words.len() {
        let mut block = [0u32; 16];
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