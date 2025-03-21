use crate::hamming::bit::BitsIterator;
use rand::{Rng, rng};
use std::fmt::{Display, Formatter};

pub mod bit;

#[derive(Clone)]
pub struct HammingCode {
    pub parity_bits: usize,
    pub total_bits: usize,
    pub block_size: usize,
    pub code: Vec<Vec<u8>>,
}

impl HammingCode {
    pub fn error_correct(&mut self) {
        self.code
            .iter_mut()
            .for_each(|block| Self::error_correct_block(block));
    }

    fn error_correct_block(block: &mut [u8]) {
        let mut parity_check = 0;
        for bit_pos in 0..block.len() * 8 {
            let block_index = bit_pos / 8;
            if block[block_index] & (1 << (bit_pos % 8)) > 0 {
                parity_check ^= bit_pos + 1;
            }
        }

        if parity_check != 0 {
            let block_index = (parity_check - 1) / 8;
            if let Some(byte) = block.get_mut(block_index) {
                *byte ^= 1 << ((parity_check - 1) % 8);
            }
        }
    }

    pub fn to_continuous(self) -> Vec<u8> {
        self.code.into_iter().flatten().collect()
    }

    pub fn from_continuous(code: Vec<u8>, block_size: usize) -> Self {
        let bytes_per_block = block_size / 8;
        let c = code
            .chunks(bytes_per_block)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<Vec<u8>>>();

        let data_bits = data_bits(block_size).unwrap();
        HammingCode {
            total_bits: block_size * c.len(),
            block_size,
            parity_bits: (block_size - data_bits) * c.len(),
            code: c,
        }
    }

    pub fn extract(self, data_bits: usize) -> Vec<u8> {
        let mut result = vec![0x00u8; (data_bits as f32 / 8f32).ceil() as usize];
        let bits = self
            .code
            .iter()
            .flatten()
            .cloned()
            .bits()
            .enumerate()
            .filter(|(i, _)| !(i % self.block_size + 1).is_power_of_two())
            .map(|(_, bit)| bit)
            .take(data_bits)
            .collect::<Vec<u8>>();

        for (i, bit) in bits.iter().enumerate() {
            result[i / 8] |= bit << (i % 8);
        }

        result
    }
}

impl Display for HammingCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for byte in self.code.iter().flatten().rev() {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

pub fn encode(data: &[u8], block_size: usize) -> Option<HammingCode> {
    let bits = data.len() * 8;
    let data_bits_per_block = data_bits(block_size)?;
    let block_count = (bits as f32 / data_bits_per_block as f32).ceil() as usize;

    let mut blocks = vec![vec![0u8; block_size / 8]; block_count];

    blocks.iter_mut().fold(0, |mut starting_bit, block| {
        encode_block(block, data, starting_bit);
        starting_bit += data_bits_per_block;
        starting_bit
    });

    Some(HammingCode {
        code: blocks,
        block_size,
        total_bits: block_count * block_size,
        parity_bits: (block_size - data_bits_per_block) * block_count,
    })
}

fn encode_block(hamming_block: &mut [u8], data: &[u8], starting_bit: usize) {
    let mut current_data_bit = starting_bit;
    for bit in 0..hamming_block.len() * 8 {
        if !(bit + 1).is_power_of_two() {
            let block_index = (bit as f32 / 8.0).floor() as usize;
            if let Some(data_bit) = bit_at(data, current_data_bit) {
                hamming_block[block_index] |= data_bit << (bit % 8);
                current_data_bit += 1;
            }
        }
    }

    let mut parity_bits = 0;
    for bit_pos in 0..hamming_block.len() * 8 {
        if let Some(bit) = bit_at(hamming_block, bit_pos) {
            if bit > 0 {
                parity_bits ^= bit_pos + 1;
            }
        }
    }

    for bit_pos in 0..hamming_block.len() * 8 {
        if (bit_pos + 1).is_power_of_two() {
            let block = (bit_pos as f32 / 8.0).floor() as usize;
            hamming_block[block] |= ((parity_bits & 1) << (bit_pos % 8)) as u8;
            parity_bits >>= 1;
        }
    }
}

fn bit_at(data: &[u8], pos: usize) -> Option<u8> {
    let block = (pos as f32 / 8.0).floor() as usize;
    data.get(block).map(|byte| (byte >> (pos % 8)) & 0x01)
}

fn data_bits(block_size: usize) -> Option<usize> {
    match block_size {
        8 => Some(4),
        16 => Some(11),
        32 => Some(26),
        64 => Some(57),
        128 => Some(120),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoding() {
        let data = [0xC5u8, 0xA3u8];
        let c = encode(&data, 32).unwrap();
        assert_eq!(c.code.len(), 1);
        assert_eq!(c.code[0], vec![0x2F, 0x3C, 0x14, 0x00])
    }

    #[test]
    fn test_hamming_extract() {
        let data = [0xAAu8, 0x55u8];
        let code = encode(&data, 32).unwrap();
        let comp = code.extract(data.len() * 8);
        assert_eq!(data.len(), comp.len());
        assert_eq!(comp, vec![0xAAu8, 0x55u8]);
    }
}
