use std::hash::{DefaultHasher, Hash, Hasher};
use crate::{encryption, hamming};

pub struct HelperData {
    pub key_hash: u64,
    pub w: Vec<u8>,
}

pub fn enroll(biometric_sample: &[u8]) -> Option<HelperData> {
    let key = encryption::generate_key_128();
    println!("S\t\t{}", hamming::to_hex_string(&key));

    let c = match hamming::encode(&key) {
        Ok(x) => x,
        Err(_) => return None,
    };
    let w = fuse(biometric_sample, &c);
    Some(HelperData {
        key_hash: hash(&key),
        w,
    })
}

pub fn verify(biometric_sample: &[u8], helper_data: &HelperData) -> bool {
    let c_prime = fuse(biometric_sample, &helper_data.w);
    let s_prime = hamming::error_correct(&c_prime);
    println!("S'\t\t{}", hamming::to_hex_string(&s_prime));
    println!("h(S')\t\t{:X}", hash(&s_prime));
    hash(&s_prime) == helper_data.key_hash
}

fn hash(key: &[u8]) -> u64 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish()
}

fn fuse(x: &[u8], y: &[u8]) -> Vec<u8> {
    x.iter().zip(y.iter()).map(|(&a, &b)| a ^ b).collect()
}