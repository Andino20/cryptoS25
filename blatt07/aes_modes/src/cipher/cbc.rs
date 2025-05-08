use aes::Aes128;
use aes::cipher::{BlockDecrypt, BlockEncrypt};
use aes::cipher::consts::U16;
use aes::cipher::generic_array::GenericArray;

pub fn encrypt(mut msg: Vec<u8>, cipher: Aes128, iv: [u8; 16]) -> Vec<u8> {
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

pub fn decrypt(mut msg: Vec<u8>, cipher: Aes128, iv: [u8; 16]) -> Vec<u8> {
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