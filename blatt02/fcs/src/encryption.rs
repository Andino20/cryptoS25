use rand::{RngCore, rng};

pub fn generate_key_128() -> [u8; 16] {
    let mut key: [u8; 16] = [0; 16];
    rng().fill_bytes(&mut key);
    key
}
