// Bytes to hex: goes from message received in bytes to hexdecimal output.
pub fn bytes_to_hex(bytes: &[u8]) -> Vec<u8> {
    let hex_table = b"0123456789abcdef";
    let mut out = Vec::with_capacity(bytes.len() * 2);

    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];

        // High nibble
        let hi = (b >> 4) & 0x0f;
        out.push(hex_table[hi as usize]);

        // Low nibble
        let lo = b & 0x0f;
        out.push(hex_table[lo as usize]);

        i += 1;
    }

    out
}