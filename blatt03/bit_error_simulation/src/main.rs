use bit_error_simulation::hamming;
use rand::{Rng, rng};
use std::fmt::Write;

fn main() {
    let key = [0xFFu8; 16];
    let hamming = hamming::encode(&key, 32).unwrap();
    for block in hamming.code {
        println!("{}", to_bit_string(&block));
    }
    println!("Total message size: {} bits", hamming.total_bits);
    println!("Parity Bits: {}", hamming.parity_bits);
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

fn burst_error(code: &[u8], starting_bit: usize, length: usize) -> Vec<u8> {
    let mut code = code.to_vec();

    for bit in (starting_bit..length + starting_bit) {
        let block = (bit as f32 / 8.0).floor() as usize;
        if let Some(byte) = code.get_mut(block) {
            *byte ^= 1 << (bit % 8);
        }
    }

    code
}

fn to_bit_string(data: &[u8]) -> String {
    data.iter().rev().fold(String::new(), |mut acc, x| {
        write!(acc, "{:08b} ", x).unwrap();
        acc
    })
}
