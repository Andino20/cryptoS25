use crate::util::queue::FixedQueue;
use aes::Aes128;
use aes::cipher::consts::U16;
use aes::cipher::generic_array::GenericArray;

pub fn encrypt(mut msg: Vec<u8>, cipher: Aes128, iv: [u8; 16], block_size: u32) -> Vec<u8> {
    let mut queue: FixedQueue<u8> = FixedQueue::new(iv.len());
    iv.iter().for_each(|b| queue.push(*b));

    
    
    todo!()
}
