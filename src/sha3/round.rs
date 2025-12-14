use crate::sha3::{
    theta_func, rho_func, pi_func, chi_func, iota_func,
};

// RnD: for applying stepmapping to receive a transformed state.
pub fn rnd_func(a: &[[u64; 5]; 5], ir: usize) -> [[u64; 5]; 5] {
    let tata = theta_func(a);
    let rho = rho_func(&tata);
    let pi = pi_func(&rho);
    let shi = chi_func(&pi);
    let lota = iota_func(&shi, ir);

    lota
}