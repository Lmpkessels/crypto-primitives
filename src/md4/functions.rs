/// The 3 round nonlinear functions that take 3 32-bit words and 
/// return 1 32-bit word.
pub fn f(x: u32, y: u32, z: u32) -> u32 { x & y | !(x) & z }

pub fn g(x: u32, y: u32, z: u32) -> u32 { x & y | x & z | y & z }

pub fn h(x: u32, y: u32, z: u32) -> u32 { x ^ y ^ z }