/// Boolean functions for RIMEMD-160.
///
/// All 5 functions apply bitwise logic bit-by-bit on 32 bit words.
/// All 5 functions will be used during the hash rounds computation.
///
/// #Reference
/// All round boolean functions (1-5) are gained from:
/// [KULeuven Report AB-9601](https://homes.esat.kuleuven.be/~bosselae/ripemd160/pdf/AB-9601/AB-9601.pdf)
pub fn f1(x: u32, y: u32, z: u32) -> u32 { x ^ y ^ z }

pub fn f2(x: u32, y: u32, z: u32) -> u32 { (x & y) | (!(x) & z) }

pub fn f3(x: u32, y: u32, z: u32) -> u32 { (x | !(y)) ^ z }

pub fn f4(x: u32, y: u32, z: u32) -> u32 { (x & z) | (y & !(z)) }

pub fn f5(x: u32, y: u32, z: u32) -> u32 { x ^ (y | !(z)) }

/// The five left round functions.
///
/// Use the range of j to decide which function to take from 0..79.
pub fn f_left(j: u32, x: u32, y: u32, z: u32) -> u32 {
    match j {
        0..=15 => f1(x, y, z),
        16..=31 => f2(x, y, z),
        32..=47 => f3(x, y, z),
        48..=63 => f4(x, y, z),
        64..=79 => f5(x, y, z),
        _ => unreachable!(),
    }
}

/// The five right round functions.
///
/// Use the range of j to decide which function to take from 0..79.
/// Here the function order is reversed for the second round (79..0).
pub fn f_right(j: u32, x: u32, y: u32, z: u32) -> u32 {
    match j {
        0..=15 => f5(x, y, z),
        16..=31 => f4(x, y, z),
        32..=47 => f3(x, y, z),
        48..=63 => f2(x, y, z),
        64..=79 => f1(x, y, z),
        _ => unreachable!(),
    }
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

    #[test]
    fn test_f() {
        let expected_fn_1 = f1(23, 11, 14);
        let expected_fn_2 = f2(23, 11, 14);
        let expected_fn_3 = f3(23, 11, 14);
        let expected_fn_4 = f4(23, 11, 14);
        let expected_fn_5 = f5(23, 11, 14);        
        
        let result_zero = f_left(0, 23, 11, 14);
        let result_one = f_left(16, 23, 11, 14);
        let result_two = f_left(32, 23, 11, 14);
        let result_three = f_left(48, 23, 11, 14);
        let result_four = f_left(64, 23, 11, 14);

        assert_eq!((expected_fn_1), (result_zero));
        assert_eq!((expected_fn_2), (result_one));
        assert_eq!((expected_fn_3), (result_two));
        assert_eq!((expected_fn_4), (result_three));
        assert_eq!((expected_fn_5), (result_four));
    }

    fn test_f_reversed() {
        let expected_fn_5 = f5(23, 11, 14);        
        let expected_fn_4 = f4(23, 11, 14);
        let expected_fn_3 = f3(23, 11, 14);
        let expected_fn_2 = f2(23, 11, 14);
        let expected_fn_1 = f1(23, 11, 14);
        
        let result_zero = f_right(0, 23, 11, 14);
        let result_one = f_right(16, 23, 11, 14);
        let result_two = f_right(32, 23, 11, 14);
        let result_three = f_right(48, 23, 11, 14);
        let result_four = f_right(64, 23, 11, 14);

        assert_eq!((expected_fn_5), (result_zero));
        assert_eq!((expected_fn_4), (result_one));
        assert_eq!((expected_fn_3), (result_two));
        assert_eq!((expected_fn_2), (result_three));
        assert_eq!((expected_fn_1), (result_four));
    }
}