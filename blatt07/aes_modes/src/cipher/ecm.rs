use aes::Aes128;
use aes::cipher::BlockEncrypt;
use aes::cipher::generic_array::GenericArray;

pub fn encrypt(mut msg: Vec<u8>, cipher: Aes128) -> Vec<u8> {
    msg.chunks_exact_mut(16)
        .map(GenericArray::from_mut_slice)
        .for_each(|block| cipher.encrypt_block(block));
    msg
}

pub fn decrypt(msg: Vec<u8>, cipher: Aes128) -> Vec<u8> {
    encrypt(msg, cipher)
}
