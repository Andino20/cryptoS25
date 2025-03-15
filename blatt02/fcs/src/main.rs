use fcs::{encryption, hamming};
use rand::{Rng, rng};
use std::hash::{DefaultHasher, Hash, Hasher};

fn main() {
    let key = encryption::generate_key_128();
    let key_hash = hash(&key);

    let hamming_code = hamming::encode(&key).expect("Failed to encode key");

    println!("STEP 1 =====================================================");
    println!("Key (S):\t\t{}", hamming::to_hex_string(&key));
    println!("Hamming Code (C):\t{}", hamming::to_hex_string(&hamming_code));

    // Generate random biometric sample data with the same size as the code
    let x = rng().random_iter().take(hamming_code.len()).collect::<Vec<u8>>();

    println!("Biom. Sample (X):\t{}",  hamming::to_hex_string(&x));

    // Fuse the biometric sample and the code
    let w = fuse(&hamming_code, &x);

    println!("Helper Data (W):\t{}", hamming::to_hex_string(&w));

    //// Step 2
    println!("STEP 2 =====================================================");
    // Make a small change to y to simulate an error
    let y = random_bit_flip(&x);
    println!("Biom. Sample (Y):\t{}",  hamming::to_hex_string(&y));

    let c = fuse(&w, &y);
    println!("Codeword (C'):\t\t{}",  hamming::to_hex_string(&c));

    let s = hamming::error_correct(&c);

    let auth_hash = hash(&s);
    if auth_hash == key_hash {
        println!("Authenticated");
    } else {
        println!("Not authenticated");
    }

    println!("Generated Key (S'):\t{}", hamming::to_hex_string(&s));
}

fn hash(key: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish()
}

fn random_bit_flip(code: &[u8]) -> Vec<u8> {
    let mut code = code.to_vec();
    let i = rng().random_range(0..code.len());
    let shift = rng().random_range(0..8);
    code[i] ^= 1 << shift;
    code
}

fn generate_biometric_sample(size: usize) -> Vec<u8> {
    rng().random_iter().take(size).collect()
}

fn fuse(x: &[u8], y: &[u8]) -> Vec<u8> {
    x.iter().zip(y.iter()).map(|(&a, &b)| a ^ b).collect()
}
