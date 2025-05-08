use crate::benchmark::time;
use crate::cipher::*;
use aes::Aes128;
use aes::cipher::KeyInit;
use aes::cipher::generic_array::GenericArray;
use log::{info, warn};
use std::fmt::Display;

pub mod benchmark;
mod cipher {
    pub mod cbc;
    pub mod ecm;
    pub mod ofb;
}

pub mod util {
    pub mod queue;
}

#[derive(Clone)]
pub enum BlockCipherMode {
    ECM,
    CBC([u8; 16]),
    OFB([u8; 16], u32),
    CFB,
    CTR,
}

impl BlockCipherMode {
    pub fn short_name(&self) -> &'static str {
        match self {
            BlockCipherMode::ECM => "ECM",
            BlockCipherMode::CBC(_) => "CBC",
            BlockCipherMode::OFB(_, _) => "OFB",
            BlockCipherMode::CFB => "CFB",
            BlockCipherMode::CTR => "CTR",
        }
    }
}

impl Display for BlockCipherMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockCipherMode::ECM => write!(f, "ECM"),
            BlockCipherMode::CBC(iv) => write!(f, "CBC(IV: {})", hex::encode(iv)),
            BlockCipherMode::OFB(iv, block_size) => {
                write!(
                    f,
                    "OFB(IV: {}, Block Size: {} B)",
                    hex::encode(iv),
                    block_size
                )
            }
            BlockCipherMode::CFB => write!(f, "CFB"),
            BlockCipherMode::CTR => write!(f, "CTR"),
        }
    }
}

pub fn encrypt<T: AsRef<[u8]>>(msg: T, key: &[u8; 16], mode: BlockCipherMode) -> Vec<u8> {
    let key = GenericArray::from_slice(key);
    let mut msg = msg.as_ref().to_vec();

    if msg.len() % 16 != 0 {
        warn!("Plaintext is not long enough, add zero-padding...");
        msg.extend(std::iter::repeat_n(0, 16 - msg.len() % 16));
    }

    match mode {
        BlockCipherMode::ECM => {
            let (cipher, duration) = time!(ecm::encrypt(msg, Aes128::new(key)));
            info!("Time elapsed encrypting (ECM): {} us", duration.as_micros());
            cipher
        }
        BlockCipherMode::CBC(iv) => {
            let (cipher, duration) = time!(cbc::encrypt(msg, Aes128::new(key), iv));
            info!("Time elapsed encrypting (CBC): {} us", duration.as_micros());
            cipher
        },
        BlockCipherMode::OFB(iv, block_size) => {
            let (cipher, duration) = time!(ofb::encrypt(msg, Aes128::new(key), iv, block_size));
            info!("Time elapsed encrypting (OFB): {} us", duration.as_micros());
            cipher
        }
        _ => msg,
    }
}

pub fn decrypt<T: AsRef<[u8]>>(msg: T, key: &[u8; 16], mode: BlockCipherMode) -> Vec<u8> {
    let msg = msg.as_ref().to_vec();
    assert_eq!(msg.len() % 16, 0);
    let key = GenericArray::from_slice(key);

    match mode {
        BlockCipherMode::ECM => {
            let (plain, duration) = time!(ecm::decrypt(msg, Aes128::new(key)));
            info!("Time elapsed decrypting (ECM): {} us", duration.as_micros());
            plain
        }
        BlockCipherMode::CBC(iv) => {
            let (plain, duration) = time!(cbc::decrypt(msg, Aes128::new(key), iv));
            info!("Time elapsed decrypting (CBC): {} us", duration.as_micros());
            plain
        }
        _ => msg,
    }
}

pub fn generate_aes128_key() -> [u8; 16] {
    rand::random_iter()
        .take(16)
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

pub fn generate_aes128_iv() -> [u8; 16] {
    generate_aes128_key()
}
