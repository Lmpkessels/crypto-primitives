/// Padding message in little endian order.
pub fn little_endian_padd(msg: &[u8]) -> Vec<u8> {
    let mut bytes: Vec<u8> = msg.to_vec();

    // Append 1 for end of message.
    // And all 0s to fill the block size.
    bytes.push(0x80);
    while bytes.len() % 64 != 56 {
        bytes.push(0x00);
    }

    // Work with byte range.
    let msg_len_bits = (msg.len() as u64) * 8;
    for i in 0..8 {
        bytes.push((msg_len_bits >> (i * 8)) as u8); // little-endian
    }

    bytes
}

//TODO: Add tests.