use crate::padd_pars::{
    big_endian_padd, big_endian_pars
};

use crate::sha256::{
    schedule, compress, to_bytes
};

/// SHA-256: Pads, parses, schedules, and compresses a message into a 256-bit 
/// hash.
///
/// # Arguments
/// - `msg`: Message as a byte slice (`&[u8]`). 
///
/// # Description
/// - **Padding:** Appends a single '1' bit, then `k` zero bits so that the
///   total length ≡ 448 (mod 512). Finally appends the original message
///   length as a 64-bit big-endian integer.
/// - **Parsing:** Splits the padded message into 512-bit blocks (16 × 32-bit words).
/// - **Scheduling:** Expands each block into 64 words using the `σ0` and `σ1` functions.
/// - **Compression:** Iteratively updates the hash state across all blocks
///   to compute the final digest.
///
/// # Returns
/// A 32-byte array representing the 256-bit hash.
///
/// # Notes
/// SHA-256 supports input messages up to `2^64 − 1` bits (≈ 2.3 exabytes).
/// 
/// # Reference
/// [FIPS PUB 180-4](https://nvlpubs.nist.gov/nistpubs/fips/nist.fips.180-4.pdf)
pub fn sha256(msg: &[u8]) -> [u8; 32] {
    let padding = big_endian_padd(msg);
    let parsing = big_endian_pars(padding);
    let scheduled = schedule(parsing);
    let digest = compress(scheduled);
    let bytes = to_bytes(digest);

    bytes
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_string_and_digest() {
        let msg = b"";
        let result = sha256(msg);
        let expected = [
            0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 
            0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f, 0xb9, 0x24, 
            0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 
            0xa4, 0x95, 0x99, 0x1b, 0x78, 0x52, 0xb8, 0x55,
        ];

        assert_eq!((result), (expected));
    }

    #[test]
    fn use_one_word_and_digest() {
        let msg = b"abc";
        let result = sha256(msg);
        let expected = [
            0xba, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea, 
            0x41, 0x41, 0x40, 0xde, 0x5d, 0xae, 0x22, 0x23,
            0xb0, 0x03, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c, 
            0xb4, 0x10, 0xff, 0x61, 0xf2, 0x00, 0x15, 0xad,
        ];

        assert_eq!((result), (expected));
    }

    #[test]
    fn expanding_in_second_block_and_digest() {
        let msg = b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
        let result = sha256(msg);
        let expected = [
            0x24, 0x8d, 0x6a, 0x61, 0xd2, 0x06, 0x38, 0xb8, 
            0xe5, 0xc0, 0x26, 0x93, 0x0c, 0x3e, 0x60, 0x39, 
            0xa3, 0x3c, 0xe4, 0x59, 0x64, 0xff, 0x21, 0x67, 
            0xf6, 0xec, 0xed, 0xd4, 0x19, 0xdb, 0x06, 0xc1,
        ];

        assert_eq!((result), (expected));
    }

    #[test]
    fn test_one_million_a_and_digest() {
        let msg = b"a".repeat(1_000_000);
        let result = sha256(&msg);
        let expected = [
            0xcd, 0xc7, 0x6e, 0x5c, 0x99, 0x14, 0xfb, 0x92, 
            0x81, 0xa1, 0xc7, 0xe2, 0x84, 0xd7, 0x3e, 0x67, 
            0xf1, 0x80, 0x9a, 0x48, 0xa4, 0x97, 0x20, 0x0e, 
            0x04, 0x6d, 0x39, 0xcc, 0xc7, 0x11, 0x2c, 0xd0,
        ];

        assert_eq!((result), (expected));
    }
}