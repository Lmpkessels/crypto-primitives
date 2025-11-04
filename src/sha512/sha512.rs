use crate::utils64::{
    big_sigma0_64, big_sigma1_64, ch64, maj64, z64
};
use crate::sha512::K;

/// SHA512 Message Digestion Algorithm.
///
/// # Argument
/// Takes scheduled message as Vec<[u64; 80]>, then compression starts.
///
/// # Description
/// - Initialize (a, b, c, d, e, f, g, h), with the 8 working variables: 
///   (h0, h1, h2, h3, h4, h5, h6, h7).
/// - Implement all round operations (Big sigma 0 & 1, Small sigma 0 & 1,
///   Ch, Maj).
/// - Compute the i-th intermediate hash value H(i)
///
/// # Returns
/// Final 8-word digest as [u64; 8] (512 bits).
///
/// # Reference
/// Based on the FIPS PUB 180-4 specification:
/// [FIPS PUB 180-4](https://nvlpubs.nist.gov/nistpubs/fips/nist.fips.180-4.pdf)
fn sha512(msg_blocks: &[[u64; 80]]) -> [u64; 8] {
    // Hash values.
    let mut h0: u64 = 0x6a09e667f3bcc908;
    let mut h1: u64 = 0xbb67ae8584caa73b;
    let mut h2: u64 = 0x3c6ef372fe94f82b;
    let mut h3: u64 = 0xa54ff53a5f1d36f1;
    let mut h4: u64 = 0x510e527fade682d1;
    let mut h5: u64 = 0x9b05688c2b3e6c1f;
    let mut h6: u64 = 0x1f83d9abfb41bd6b;
    let mut h7: u64 = 0x5be0cd19137e2179;

    for w in msg_blocks {
        // Initialized working variables.
        let mut a = h0;
        let mut b = h1;
        let mut c = h2;
        let mut d = h3;
        let mut e = h4;
        let mut f = h5;
        let mut g = h6;
        let mut h = h7;

        // Implement 80 round operations.
        for t in 0..80 {

            let temp1 = z64(
                z64(
                    z64(
                        z64(
                            h, 
                            big_sigma1_64(e)), 
                        ch64(e, f, g)), 
                    K[t]), 
                w[t]
            );
            let temp2 = z64(
                big_sigma0_64(a), 
                maj64(a, b, c)
            );
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);

        }

        // Compute the i-th intermediate hash value H(i).
        h0 = z64(a, h0);
        h1 = z64(b, h1);
        h2 = z64(c, h2);
        h3 = z64(d, h3);
        h4 = z64(e, h4);
        h5 = z64(f, h5);
        h6 = z64(g, h6);
        h7 = z64(h, h7);

    }

    // Digested state.
    [
        h0,
        h1,
        h2,
        h3,
        h4,
        h5,
        h6,
        h7
    ]
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::sha512::{ schedule };
    use crate::padd_pars::{
        big_endian_padd64, big_endian_pars64        
    };

    #[test]
    fn sha512_computes_empty_string() {
        let msg = b"";
        let padded = big_endian_padd64(msg);
        let parsed = big_endian_pars64(padded);
        let scheduled = schedule(&parsed);
        
        let result = sha512(&scheduled);
        let expected = [
            0xcf83e1357eefb8bd, 
            0xf1542850d66d8007, 
            0xd620e4050b5715dc,
            0x83f4a921d36ce9ce,
            0x47d0d13c5d85f2b0,
            0xff8318d2877eec2f, 
            0x63b931bd47417a81,
            0xa538327af927da3e
        ];

        assert_eq!((result), (expected));
    }

    #[test]
    fn sha512_computes_abc() {
        let msg = b"abc";
        let padded = big_endian_padd64(msg);
        let parsed = big_endian_pars64(padded);
        let scheduled = schedule(&parsed);
        
        let result = sha512(&scheduled);
        let expected = [
            0xddaf35a193617aba,
            0xcc417349ae204131,
            0x12e6fa4e89a97ea2, 
            0x0a9eeee64b55d39a,
            0x2192992a274fc1a8, 
            0x36ba3c23a3feebbd,
            0x454d4423643ce80e,
            0x2a9ac94fa54ca49f
        ];

        assert_eq!((result), (expected));
    }

    #[test]
    fn sha512_computes_brown_fox() {
        let msg = b"The quick brown fox jumps over the lazy dog";
        let padded = big_endian_padd64(msg);
        let parsed = big_endian_pars64(padded);
        let scheduled = schedule(&parsed);
        
        let result = sha512(&scheduled);
        let expected = [
            0x07e547d9586f6a73, 
            0xf73fbac0435ed769,
            0x51218fb7d0c8d788, 
            0xa309d785436bbb64,
            0x2e93a252a954f239,
            0x12547d1e8a3b5ed6,
            0xe1bfd7097821233f,
            0xa0538f3db854fee6
        ];

        assert_eq!((result), (expected));
    }

    #[test]
    fn sha512_computes_1_000_000_as() {
        let msg = b"a".repeat(1_000_000);
        let padded = big_endian_padd64(&msg);
        let parsed = big_endian_pars64(padded);
        let scheduled = schedule(&parsed);
        
        let result = sha512(&scheduled);
        let expected = [
            0xe718483d0ce76964,
            0x4e2e42c7bc15b463,
            0x8e1f98b13b204428,
            0x5632a803afa973eb,
            0xde0ff244877ea60a,
            0x4cb0432ce577c31b,
            0xeb009c5c2c49aa2e,
            0x4eadb217ad8cc09b
        ];

        assert_eq!((result), (expected));
    }
}