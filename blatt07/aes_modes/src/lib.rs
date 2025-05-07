use crate::benchmark::time;
use aes::Aes128;
use aes::cipher::consts::U16;
use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockDecrypt, BlockEncrypt, KeyInit};
use log::info;

pub mod benchmark;

pub enum BlockCipherMode {
    ECM,
    CBC([u8; 16]),
    OFB,
    CFB,
    CTR,
}

pub fn encrypt<T: AsRef<[u8]>>(msg: T, key: &[u8; 16], mode: BlockCipherMode) -> Vec<u8> {
    let key = GenericArray::from_slice(key);
    let mut msg = msg.as_ref().to_vec();

    if msg.len() % 16 != 0 {
        msg.extend(std::iter::repeat_n(0, 16 - msg.len() % 16));
    }

    match mode {
        BlockCipherMode::ECM => {
            let (cipher, duration) = time!(encrypt_ecm(msg, Aes128::new(key)));
            info!("Time elapsed encrypting (ECM): {} us", duration.as_micros());
            cipher
        }
        BlockCipherMode::CBC(iv) => {
            let (cipher, duration) = time!(encrypt_cbc(msg, Aes128::new(key), iv));
            info!("Time elapsed encrypting  (CBC): {} us", duration.as_micros());
            cipher
        }
        _ => msg,
    }
}

fn encrypt_ecm(mut msg: Vec<u8>, cipher: Aes128) -> Vec<u8> {
    msg.chunks_exact_mut(16)
        .map(GenericArray::from_mut_slice)
        .for_each(|block| cipher.encrypt_block(block));
    msg
}

fn encrypt_cbc(mut msg: Vec<u8>, cipher: Aes128, iv: [u8; 16]) -> Vec<u8> {
    let mut prev = &GenericArray::from(iv);
    for block in msg
        .chunks_exact_mut(16)
        .map(GenericArray::<u8, U16>::from_mut_slice)
    {
        block.iter_mut().zip(prev).for_each(|(a, &b)| *a ^= b);
        cipher.encrypt_block(block);
        prev = block;
    }
    msg
}

pub fn decrypt<T: AsRef<[u8]>>(msg: T, key: &[u8; 16], mode: BlockCipherMode) -> Vec<u8> {
    let msg = msg.as_ref().to_vec();
    assert_eq!(msg.len() % 16, 0);
    let key = GenericArray::from_slice(key);

    match mode {
        BlockCipherMode::ECM => decrypt_ecm(msg, Aes128::new(key)),
        BlockCipherMode::CBC(iv) => decrypt_cbc(msg, Aes128::new(key), iv),
        _ => msg,
    }
}

fn decrypt_ecm(msg: Vec<u8>, cipher: Aes128) -> Vec<u8> {
    encrypt_ecm(msg, cipher)
}

fn decrypt_cbc(mut msg: Vec<u8>, cipher: Aes128, iv: [u8; 16]) -> Vec<u8> {
    let mut block_iter = msg
        .chunks_exact_mut(16)
        .map(GenericArray::<u8, U16>::from_mut_slice)
        .rev()
        .peekable();

    while let Some(block) = block_iter.next() {
        let prev = match block_iter.peek() {
            Some(b) => b,
            None => &GenericArray::from(iv),
        };

        cipher.decrypt_block(block);
        block.iter_mut().zip(prev).for_each(|(a, &b)| *a ^= b);
    }

    msg
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
