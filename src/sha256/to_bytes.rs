/// Convert 8 × 32-bit words (digest state) into a 32-byte array.
///
/// Each 32-bit word is split into 4 bytes in big-endian order
/// (most significant byte first).
pub fn to_bytes(digest: [u32; 8]) -> [u8; 32] {
    let mut outp = [0u8; 32];
    let mut i = 0;

    while i < 8 {
        let word = digest[i];
        let j = i * 4;

        outp[j] =     (word >> 24) as u8;
        outp[j + 1] = (word >> 16) as u8;
        outp[j + 2] = (word >> 8)  as u8;
        outp[j + 3] = (word)       as u8;

        i += 1;
    }

    outp
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn converts_mixed_words_correctly() {
        let digest = [
            0x12345678, 0x90abcdef, 0xdeadbeef, 0x00000001,
            0x01020304, 0xaabbccdd, 0x11223344, 0x55667788,
        ];
        let result = to_bytes(digest);
        let expected = [
            0x12, 0x34, 0x56, 0x78,
            0x90, 0xab, 0xcd, 0xef,
            0xde, 0xad, 0xbe, 0xef,
            0x00, 0x00, 0x00, 0x01,
            0x01, 0x02, 0x03, 0x04,
            0xaa, 0xbb, 0xcc, 0xdd,
            0x11, 0x22, 0x33, 0x44,
            0x55, 0x66, 0x77, 0x88,
        ];

        assert_eq!(result, expected);
    }
}