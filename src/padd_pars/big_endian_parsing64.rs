pub fn big_endian_pars64(msg_in_bytes: Vec<u8>) -> Vec<[u64; 16]> {
    let mut words: Vec<u64> = Vec::new();
    let mut j = 0;
    while j < msg_in_bytes.len() {
        let b0 = msg_in_bytes[j] as u64;
        let b1 = msg_in_bytes[j + 1] as u64;
        let b2 = msg_in_bytes[j + 2] as u64;
        let b3 = msg_in_bytes[j + 3] as u64;
        let b4 = msg_in_bytes[j + 4] as u64;
        let b5 = msg_in_bytes[j + 5] as u64;
        let b6 = msg_in_bytes[j + 6] as u64;
        let b7 = msg_in_bytes[j + 7] as u64;
        let word = 
        (b0 << 56) | (b1 << 48) | (b2 << 40) | (b3 << 32) 
        | 
        (b4 << 24) | (b5 << 16) | (b6 << 8) | (b7);
        words.push(word);
        j += 8
    }
    let mut blocks: Vec<[u64; 16]> = Vec::new();
    let mut k = 0;
    while k < words.len() {
        let mut block = [0u64; 16];
        let mut l = 0;
        // The wrong index was used since k is infinite in terms of vector
        // values stored (u64) block never gets the right index.
        while l < 16 {
            block[l] = words[k + l];
            l += 1;
        }
        blocks.push(block);
        k += 16;
    }
    blocks
}