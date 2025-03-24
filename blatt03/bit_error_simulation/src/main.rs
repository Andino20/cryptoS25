use bit_error_simulation::hamming;
use bit_error_simulation::hamming::HammingCode;
use rand::{Rng, rng};
use std::fmt::Write;
use std::hash::{DefaultHasher, Hash, Hasher};

fn main() {
    println!(
        "FLIPS\t(8,4)\t(16,11)\t(32,26)\t(64,57)\t(128,120)\n-------------------------------------------"
    );
    for flip_amount in 1..=15 {
        print!("{flip_amount}\t");
        for block_size in [8, 16, 32, 64, 128] {
            let successful_attempts = (0..50)
                .map(|_| fcs_run(block_size, flip_amount))
                .filter(|&r| r)
                .count();
            print!("{successful_attempts}\t");
        }
        println!();
    }
}

fn fcs_run(hamming_block_size: usize, flip_amount: usize) -> bool {
    let s = rng().random_iter().take(16).collect::<Vec<u8>>();
    let hs = hash(&s);
    let c = hamming::encode(&s, hamming_block_size)
        .expect("encode failed")
        .to_continuous();

    //println!("S\t{}", to_hex_string(&s));
    //println!("C\t{}", to_hex_string(&c));
    //println!("h(S)\t{:x}", hs);

    let x = rng().random_iter().take(s.len()).collect::<Vec<u8>>();
    let x = hamming::encode(&x, hamming_block_size)
        .expect("encode failed")
        .to_continuous();
    //println!("X\t{}", to_hex_string(&x));

    let w = fuse(&c, &x);
    //println!("W\t{}", to_hex_string(&w));

    //println!("------------------------------------------------");
    //let y = random_bit_flip(&x, flip_amount);
    let y = random_bit_flip_range(&x, 0..s.len()*8, flip_amount);
    //let y = burst_error(&x, rng().random_range(0..s.len()*8), flip_amount);
    //println!("Y\t{}", to_hex_string(&y));

    let c_prime = fuse(&w, &y);
    //println!("C'\t{}", to_hex_string(&c_prime));

    let mut ecc = HammingCode::from_continuous(c_prime, hamming_block_size);
    ecc.error_correct();

    let s_prime = ecc.extract(s.len() * 8);
    //println!("S'\t{}", to_hex_string(&s_prime));

    let hs_prime = hash(&s_prime);
    //println!("h(S')\t{:x}", hs_prime);

    hs == hs_prime
}

fn random_bit_flip(code: &[u8], amount: usize) -> Vec<u8> {
    let mut code = code.to_vec();
    for _ in 0..amount {
        let i = rng().random_range(0..code.len());
        let shift = rng().random_range(0..8);
        code[i] ^= 1 << shift;
    }
    code
}

fn random_bit_flip_range(code: &[u8], bit_range: std::ops::Range<usize>, amount: usize) -> Vec<u8> {
    let mut code = code.to_vec();
    for _ in 0..amount {
        let bit = rng().random_range(bit_range.clone());
        let block = bit / 8;
        let shift = rng().random_range(0..8);
        code[block] ^= 1 << shift;
    }
    code
}

fn fuse(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b).map(|(&a, &b)| a ^ b).collect()
}

fn burst_error(code: &[u8], starting_bit: usize, length: usize) -> Vec<u8> {
    let mut code = code.to_vec();
    for bit in starting_bit..length + starting_bit {
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

fn to_hex_string(data: &[u8]) -> String {
    data.iter().rev().fold(String::new(), |mut acc, x| {
        write!(acc, "{:02x}", x).unwrap();
        acc
    })
}

fn hash(data: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::default();
    data.hash(&mut hasher);
    hasher.finish()
}
