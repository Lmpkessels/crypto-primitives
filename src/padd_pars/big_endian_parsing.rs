/// Parse a padded message into 512-bit blocks of 16 × 32-bit words.
///
/// # Description
/// - Groups every 4 bytes into one 32-bit word (big-endian).
/// - Collects 16 words into a 512-bit block.
/// - Expands into multiple blocks if the message length exceeds 512 bits.
///
/// # Returns
/// A `Vec<[u32; 16]>`, where each element is one 512-bit block.
//TODO: Change name to BIG-ENDIAN-PARS.
pub fn big_endian_pars(bytes: Vec<u8>) -> Vec<[u32; 16]> {
    
    let mut words: Vec<u32> = Vec::new();
    let mut j = 0;
    // Convert every 4 bytes into a 32-bit big-endian word.
    while j < bytes.len() {
        let b0 = bytes[j] as u32;
        let b1 = bytes[j + 1] as u32;
        let b2 = bytes[j + 2] as u32;
        let b3 = bytes[j + 3] as u32;

        let word = (b0 << 24) | (b1 << 16) | (b2 << 8) | (b3);

        words.push(word);
    
        // += 4 to work with 4 bytes.
        j += 4;
    } 

    let mut blocks: Vec<[u32; 16]> = Vec::new();
    let mut k = 0;
    // Group words into 16-word (512-bit) blocks.
    while k < words.len() {
        let mut block = [0u32; 16];
        let mut l = 0;
        while l < 16 {
            block[l] = words[k + l];
            // += 1 to work in word range.
            l += 1
        }
        blocks.push(block);
        // += 16 to work in block range. 
        k += 16;
    }

    blocks
}

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

#[cfg(test)]
mod test {
    use super::*;
    use crate::padd_pars::{little_endian_padd, big_endian_padd};

    #[test] 
    fn pads_message_to_16_32_bit_words() {
        let bytes = little_endian_padd(b"abc"); 
        let result = little_endian_pars(bytes); 

        let expected_block: [u32; 16] = [
            0x80636261, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000018, 0x00000000
        ];
        let expected: Vec<[u32; 16]> = vec![expected_block];

        assert_eq!((result), (expected));
    }

    #[test]
    fn pads_message_to_16_32_bit_words_and_expands_into_second_block() {
        let bytes = big_endian_padd(b"AAAAA_AAAAA_AAAAA_AAAAA_AAAAA_AAAAA_\
        AAAAA_AAAAA_AAAAA_AAAAA_AAAAA");
        let result = big_endian_pars(bytes);

        // First 512-bit block
        let first_block: [u32; 16] = [
            1094795585, 1096761665, 1094795615, 1094795585, 
            1096761665, 1094795615, 1094795585, 1096761665, 
            1094795615, 1094795585, 1096761665, 1094795615, 
            1094795585, 1096761665, 1094795615, 1094795585
        ];

        // Second 512-bit block (from padding + length)
        let second_block: [u32; 16] = [
            1098907648, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 520
        ];

        let expected: Vec<[u32; 16]> = vec![first_block, second_block];

        assert_eq!(result, expected);
    }

    #[test] 
    fn small_endian_pars_test() {
        let bytes = little_endian_padd(b"abc"); 
        let result = little_endian_pars(bytes); 

        let expected_block: [u32; 16] = [
            0x80636261, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000000, 0x00000000,
            0x00000000, 0x00000000, 0x00000018, 0x00000000
        ];
        let expected: Vec<[u32; 16]> = vec![expected_block];

        assert_eq!((result), (expected));
    }
}
