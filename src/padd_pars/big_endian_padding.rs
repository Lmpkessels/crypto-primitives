/// Pad a message to a 512-bit block (SHA-256 padding).
///
/// # Arguments
/// - `msg`: Message as a byte slice (`&[u8]`).  
///
/// # Description
/// Appends padding according to the SHA-256 specification:  
/// - Append a single `1` bit (0x80).  
/// - Append `0` bits until the length is congruent to 56 (mod 64).  
/// - Append the original message length as a 64-bit big-endian integer.  
///
/// # Returns
/// A `Vec<u8>` containing the padded message in bytes.
pub fn big_endian_padd(msg: &[u8]) -> Vec<u8> {
    let mut bytes: Vec<u8> = msg.to_vec();

    // Append 1 + k.
    bytes.push(0x80);
    while bytes.len() % 64 != 56 {
        bytes.push(0x00);
    }

    let msg_as_bits = (msg.len() as u64) * 8;
    let mut i = 0;
    // Append MSG length, in big endian order.
    while i < 8 {
        // Shift by (7 - i) * 8 to extract big-endian length bytes (MSB first).
        let decremental_shift = (7 - i) * 8;
        let message = ((msg_as_bits >> decremental_shift) & 0xFF) as u8;

        bytes.push(message);
        i += 1;
    }

    bytes
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn takes_msg_appends_1_k_and_msg_length_computes_to_vec_of_bytes() {
        let msg = b"abc";

        let result = big_endian_padd(msg);
        let expected = [
            97, 98, 99, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
            0, 0, 24
        ];
        
        assert_eq!((result), (expected));
    }
}