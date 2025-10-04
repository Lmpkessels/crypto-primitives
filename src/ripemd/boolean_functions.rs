/// Boolean functions for RIMEMD-160.
///
/// All 5 functions apply bitwise logic bit-by-bit on 32 bit words.
/// All 5 functions will be used during the hash rounds computation.
fn f1(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

fn f2(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (!(x) & z)
}

fn f3(x: u32, y: u32, z: u32) -> u32 {
    (x | !(y)) ^ z
}

fn f4(x: u32, y: u32, z: u32) -> u32 {
    (x & z) | (y & !(z))
}

fn f5(x: u32, y: u32, z: u32) -> u32 {
    x ^ (y | !(z))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn compute_f1_return_x_xor_y_xor_z() {
        let x = 32;
        let y = 18;
        let z = 122;

        let result = f1(x, y, z);
        let expected = x ^ y ^ z;

        assert_eq!((result), (expected));
    }

    #[test]
    fn compute_f2_return_x_and_y_or_not_x_and_z() {
        let x = 68;
        let y = 99;
        let z = 128;

        let result = f2(x, y, z);
        let expected = (x & y) | (!(x) & z);

        assert_eq!((result), (expected));
    }

    #[test]
    fn compute_f3_return_x_or_not_y_xor_z() {
        let x = 234;
        let y = 123;
        let z = 321;

        let result = f3(x, y, z);
        let expected = (x | !(y)) ^ z;

        assert_eq!((result), (expected));
    }

    #[test]
    fn compute_f4_return_x_and_z_or_y_and_not_z() {
        let x = 128;
        let y = 45;
        let z = 512;
        
        let result = f4(x, y, z);
        let expected = (x & z) | (y & !(z));

        assert_eq!((result), (expected));
    }

    #[test]
    fn compute_f5_return_x_xor_y_or_not_z() {
        let x = 9;
        let y = 30;
        let z = 8;

        let result = f5(x, y, z);
        let expected = x ^ (y | !(z));

        assert_eq!((result), (expected));
    }
}