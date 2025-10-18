/// The 4 nonlinear functions (F, G, H, I) working on 32-bit words with 
/// bitwise logic.
pub fn f(x: u32, y: u32, z: u32) -> u32 { (x & y) | (!(x) & z) }
pub fn g(x: u32, y: u32, z: u32) -> u32 { (x & z) | (y & !(z)) }
pub fn h(x: u32, y: u32, z: u32) -> u32 { x ^ y ^ z }
pub fn i(x: u32, y: u32, z: u32) -> u32 { y ^ (x | !(z)) }