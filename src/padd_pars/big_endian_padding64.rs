pub fn big_endian_padd64(msg: &[u8]) -> Vec<u8> {
    let mut bytes: Vec<u8> = msg.to_vec();
    // Append 1.
    bytes.push(0x80);
    // Append all 0s (k) till the message is l + 1 + k -= 896 MOD 1024.
    while bytes.len() % 128 != 112 {
        bytes.push(0x00);
    }
    // Append message when l + 1 + k -= 896 MOD 1024.
    let msg_in_bits = (msg.len() as u128) * 8;
    let mut i = 0;
    while i < 16 {
        let decremental_shift = (15 - i) * 8;
        let message_length_byte = (msg_in_bits >> decremental_shift & 0xff) as u8;
        bytes.push(message_length_byte);
        i += 1;
    }
    bytes
}