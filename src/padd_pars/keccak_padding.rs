// Keccak padding: to produce output string of decired length.
pub fn keccak_padd(mut v: Vec<u8>, rate: usize) -> Vec<u8> {
    let rate_bytes = rate / 8;

    // Domain sepperator.
    v.push(0x06);

    // Pad till end of block with all 0s.
    while (v.len() % rate_bytes) != rate_bytes - 1 {
        v.push(0x00);
    }

    // Final byte, end of message.
    v.push(0x80);

    v
}