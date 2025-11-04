use crate::utils::{ rotl, z };
use crate::sha1::{ k, f };

/// SHA1 Message Digestion Algorithm.
///
/// # Argument
/// Takes scheduled message as Vec<[u32; 80]>, then compression starts.
///
/// # Description
/// - Initialize (a, b, c, d, e), with the 5 working variables; 
///   (h0, h1, h2, h3, h4).
/// - Implement all round operations (Ch, Maj, Parity, ROTL).
/// - Compute the i-th intermediate hash value H(i)
///
/// # Returns
/// Final 8-word digest as [u32; 5].
///
/// # Reference
/// Based on the FIPS PUB 180-4 specification:
/// [FIPS PUB 180-4](https://nvlpubs.nist.gov/nistpubs/fips/nist.fips.180-4.pdf)
fn sha1(msg: &[[u32; 80]]) -> [u32; 5] {
    // Initialize working variables.
    let mut h0: u32 = 0x67452301;
    let mut h1: u32 = 0xefcdab89;
    let mut h2: u32 = 0x98badcfe;
    let mut h3: u32 = 0x10325476;
    let mut h4: u32 = 0xc3d2e1f0;

    for m in msg {
        let mut a = h0;
        let mut b = h1;
        let mut c = h2;
        let mut d = h3;
        let mut e = h4;

        // Implement round operations.
        for t in 0..80 {

            let temp = z(
                z(
                    z(
                        z(
                            rotl(a, 5), 
                            f(t, b, c, d)), 
                        e), 
                    k(t)), 
                m[t as usize]
            );

            e = d;
            d = c;
            c = rotl(b, 30);
            b = a;
            a = temp;

        };

        // Compute the i-th intermediate hash value.
        h0 = z(a, h0);
        h1 = z(b, h1);
        h2 = z(c, h2);
        h3 = z(d, h3);
        h4 = z(e, h4);

    }

    // Digested state.
    [
        h0, 
        h1, 
        h2, 
        h3, 
        h4
    ]

}

#[cfg(test)]
mod test {
    use super::*;
    use crate::padd_pars::{
        big_endian_padd, big_endian_pars
    };
    use crate::sha1::schedule;

    #[test]
    fn sha1_computes_empty_string() {
        let msg = b"";
        let padded = big_endian_padd(msg);
        let parsed = big_endian_pars(padded);
        let scheduled = schedule(&parsed);
        
        let result = sha1(&scheduled);

        let expected = [
            0xda39a3ee, 0x5e6b4b0d, 0x3255bfef, 0x95601890, 0xafd80709
        ];

        assert_eq!((result), (expected));
    }

    #[test]
    fn sha1_computes_abc() {
        let msg = b"abc";
        let padded = big_endian_padd(msg);
        let parsed = big_endian_pars(padded);
        let scheduled = schedule(&parsed);
        
        let result = sha1(&scheduled);

        let expected = [
            0xa9993e36, 0x4706816a, 0xba3e2571, 0x7850c26c, 0x9cd0d89d
        ];

        assert_eq!((result), (expected));
    }

    #[test]
    fn sha1_computes_448bits() {
        let msg = b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
        let padded = big_endian_padd(msg);
        let parsed = big_endian_pars(padded);
        let scheduled = schedule(&parsed);
        
        let result = sha1(&scheduled);

        let expected = [
            0x84983e44, 0x1c3bd26e, 0xbaae4aa1, 0xf95129e5, 0xe54670f1
        ];

        assert_eq!((result), (expected));
    }

    #[test]
    fn sha1_computes_8_000_000bits() {
        let msg = b"a".repeat(1_000_000);
        let padded = big_endian_padd(&msg);
        let parsed = big_endian_pars(padded);
        let scheduled = schedule(&parsed);
        
        let result = sha1(&scheduled);

        let expected = [
            0x34aa973c, 0xd4c4daa4, 0xf61eeb2b, 0xdbad2731, 0x6534016f
        ];

        assert_eq!((result), (expected));
    }

    #[test]
    fn sha1_computes_896bits() {
        let msg = b"abcdefghbcdefghicdefghijdefghijkefghijklfghijklmghijklmn\
                    hijklmnoijklmnopjklmnopqklmnopqrlmnopqrsmnopqrstnopqrstu"
        ;
        let padded = big_endian_padd(msg);
        let parsed = big_endian_pars(padded);
        let scheduled = schedule(&parsed);
        
        let result = sha1(&scheduled);

        let expected = [
            0xa49b2446, 0xa02c645b, 0xf419f995, 0xb6709125, 0x3a04a259
        ];

        assert_eq!((result), (expected));
    }

    #[test]
    fn sha1_computes_brown_fox_over_dog() {
        let msg = b"The quick brown fox jumps over the lazy dog";
        let padded = big_endian_padd(msg);
        let parsed = big_endian_pars(padded);
        let scheduled = schedule(&parsed);
        
        let result = sha1(&scheduled);

        let expected = [
            0x2fd4e1c6, 0x7a2d28fc, 0xed849ee1, 0xbb76e739, 0x1b93eb12
        ];

        assert_eq!((result), (expected));
    }

    #[test]
    fn sha1_computes_brown_fox_over_cog() {
        let msg = b"The quick brown fox jumps over the lazy cog";
        let padded = big_endian_padd(msg);
        let parsed = big_endian_pars(padded);
        let scheduled = schedule(&parsed);
        
        let result = sha1(&scheduled);

        let expected = [
            0xde9f2c7f, 0xd25e1b3a, 0xfad3e85a, 0x0bd17d9b, 0x100db4b3
        ];

        assert_eq!((result), (expected));
    }
}