use bit_error_simulation::fcs::{enroll, verify};
use bit_error_simulation::hamming;
use rand::{Rng, rng};

fn main() {
    println!("VARIABLE\tVALUE");

    // Generate random biometric sample data with the same size as the code
    let x = rng().random_iter().take(17).collect::<Vec<u8>>();
    println!("X\t\t{}", hamming::to_hex_string(&x));

    let helper = enroll(&x).expect("Failed to enroll biometric sample.");

    println!("h(S):\t\t{:X}", helper.key_hash);

    /*
    for _ in 0..1 {
        let y = burst_error(&x, 10);
        println!("Y\t\t{}", hamming::to_hex_string(&y));

        if verify(&y, &helper) {
            println!("Authenticated!");
        } else {
            println!("Not authenticated!");
        }
    }
     */

    let x: [u8; 1] = [0xFF];
    for _ in 0..10 {
        let y = burst_error(&x, 4);
        println!("y: {:08b}", y[0]);
    }
}

fn random_bit_flip(code: &[u8], amount: u32) -> Vec<u8> {
    let mut code = code.to_vec();

    for _ in 0..amount {
        let i = rng().random_range(0..code.len());
        let shift = rng().random_range(0..8);
        code[i] ^= 1 << shift;
    }

    code
}

fn burst_error(code: &[u8], length: usize) -> Vec<u8> {
    let mut code = code.to_vec();

    let starting_pos = rng().random_range(0..code.len() * 8);
    for bit in (starting_pos..length + starting_pos) {
        let block = (bit as f32 / 8.0).floor() as usize;
        if let Some(byte) = code.get_mut(block) {
            *byte ^= 1 << (bit % 8);
        }
    }

    code
}
