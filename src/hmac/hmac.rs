use crate::sha256::sha256;
use crate::hmac::get_right_block_size;

/// HMAC (Hash-based Message Authentication Code) using SHA-256.
/// 
/// # Arguments
/// - `k`: Key as a byte slice (`&[u8]`).  
/// - `m`: Message as a byte slice (`&[u8]`).  
///
/// # Description
/// - The key is XORed with the inner pad (ipad) and the outer pad (opad).  
/// - The message is appended to the ipad and hashed to produce the inner hash.  
/// - That inner hash is then appended to the opad and hashed again, producing 
///   the final tag.  
///
/// # Returns
/// A 32-byte array containing the authentication tag (MAC).
///
/// # References
/// - [RFC 2104](https://www.rfc-editor.org/rfc/rfc2104)  
/// - [RFC 4231](https://www.rfc-editor.org/rfc/rfc4231)
pub fn hmac(k: &[u8], m: &[u8]) -> [u8; 32] {
    let mut result: Vec<u8> = Vec::new();
    let mut ipad: Vec<u8> = vec![0x36u8; 64];
    let mut opad: Vec<u8> = vec![0x5cu8; 64];

    // Vector ipad[i] and opad[i] XORed with k[i].
    for i in 0..64 {
        ipad[i] ^= k[i];
        opad[i] ^= k[i];
    };

    let msg: Vec<u8> = m.to_vec();
    let msg_len = m.len();

    // ((K0 ^ ipad) || text)).
    let mut j = 0;
    while j < msg_len {
        ipad.push(msg[j]);
        j += 1;
    };

    // H((K0 ^ ipad) || text)).
    let sha = sha256(&ipad);

    // ((K0 ^ opad )|| H((K0 ^ ipad) || text))
    for k in 0..32 {
        opad.push(sha[k]);
    };
    
    // h((K0 ^ opad )|| H((K0 ^ ipad) || text)).
    sha256(&opad)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::hmac::get_right_block_size;

    #[test]
    fn computes_the_right_vector() {
        let key = b"abcdef";
        let result = get_right_block_size(key);

        let expected = [
            97, 98, 99, 100, 101, 102, 00, 00, 
            00, 00, 00, 00, 00, 00, 00, 00, 
            00, 00, 00, 00, 00, 00, 00, 00,  
            00, 00, 00, 00, 00, 00, 00, 00, 
            00, 00, 00, 00, 00, 00, 00, 00, 
            00, 00, 00, 00, 00, 00, 00, 00, 
            00, 00, 00, 00, 00, 00, 00, 00, 
            00, 00, 00, 00, 00, 00, 00, 00,
        ];
        
        assert_eq!((result), (expected));
    }

    #[test]
    fn brown_fox() {
        let k = get_right_block_size(b"key");
        let msg = b"The quick brown fox jumps over the lazy dog"; 

        let result = hmac(&k, msg);
        let expected = [
            0xf7, 0xbc, 0x83, 0xf4, 0x30, 0x53, 0x84, 0x24, 
            0xb1, 0x32, 0x98, 0xe6, 0xaa, 0x6f, 0xb1, 0x43,
            0xef, 0x4d, 0x59, 0xa1, 0x49, 0x46, 0x17, 0x59,
            0x97, 0x47, 0x9d, 0xbc, 0x2d, 0x1a, 0x3c, 0xd8,
        ];

        assert_eq!((result), (expected));
    }

    #[test]
    fn long_key_vector() {
        let long_key: Vec<u8> = vec![0xaa; 131];

        let k = get_right_block_size(&long_key);
        let msg = b"Test Using Larger Than Block-Size Key - Hash Key First";

        let result = hmac(&k, msg);
        let expected = [
            0x60, 0xe4, 0x31, 0x59, 0x1e, 0xe0, 0xb6, 0x7f,
            0x0d, 0x8a, 0x26, 0xaa, 0xcb, 0xf5, 0xb7, 0x7f,
            0x8e, 0x0b, 0xc6, 0x21, 0x37, 0x28, 0xc5, 0x14,
            0x05, 0x46, 0x04, 0x0f, 0x0e, 0xe3, 0x7f, 0x54,
        ];

        assert_eq!((result), (expected));
    }

    #[test]
    fn rfc4231_case6_long_key_long_msg() {
        let long_key = vec![0xaa; 131];
        let k = get_right_block_size(&long_key);
        let msg = b"This is a test using a larger than block-size key and a \
        larger than block-size data. The key needs to be hashed before being \
        used by the HMAC algorithm.";
        let mac = hmac(&k, msg);
        let expected = [
            0x9b, 0x09, 0xff, 0xa7, 0x1b, 0x94, 0x2f, 0xcb,
            0x27, 0x63, 0x5f, 0xbc, 0xd5, 0xb0, 0xe9, 0x44,
            0xbf, 0xdc, 0x63, 0x64, 0x4f, 0x07, 0x13, 0x93,
            0x8a, 0x7f, 0x51, 0x53, 0x5c, 0x3a, 0x35, 0xe2,
        ];
        assert_eq!(mac, expected);
    }
}