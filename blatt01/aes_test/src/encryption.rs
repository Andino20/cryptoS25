use aes::Aes128;
use rand::{RngCore, rng};

pub type Aes128CbcEnc = cbc::Encryptor<Aes128>;
//pub type Aes128CbcDec = cbc::Decryptor<Aes128>;

pub fn generate_aes128_key() -> [u8; 16] {
    let mut key: [u8; 16] = [0; 16];
    rng().fill_bytes(&mut key);
    key
}

pub fn generate_aes128_iv() -> [u8; 16] {
    let mut iv: [u8; 16] = [0; 16];
    rng().fill_bytes(&mut iv);
    iv
}
