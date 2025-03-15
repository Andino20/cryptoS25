use rand::{RngCore, rng};

pub fn generate_aes128_key() -> String {
    let mut key: [u8; 16] = [0; 16];
    rng().fill_bytes(&mut key);
    key.iter()
        .map(|&byte| byte as char)
        .collect::<String>()
}
