pub struct HammingCode {
    pub parity_bits: usize,
    pub total_bits: usize,
    pub code: Vec<Vec<u8>>,
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
        total_bits: block_count * block_size,
        parity_bits: (block_size - data_bits_per_block) * block_count,
    })
}

fn encode_block(hamming_block: &mut [u8], data: &[u8], starting_bit: usize) {
    let mut current_data_bit = starting_bit;
    for bit in 0..hamming_block.len() * 8 {
        if !(bit + 1).is_power_of_two() {
            let block_index = (bit as f32 / 8.0).floor() as usize;
            if let Some(data_bit) = bit_at(&data, current_data_bit) {
                hamming_block[block_index] |= data_bit << (bit % 8);
                current_data_bit += 1;
            }
        }
    }

    let mut parity_bits = 0;
    for bit_pos in 0..hamming_block.len() * 8 {
        if let Some(bit) = bit_at(&hamming_block, bit_pos) {
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
    if let Some(byte) = data.get(block) {
        Some(byte >> (pos % 8) & 0x01)
    } else {
        None
    }
}

fn data_bits(code_size: usize) -> Option<usize> {
    match code_size {
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
    use crate::hamming::*;

    #[test]
    fn test_encode() {
        let data = [0xFF; 1];
    }
}
