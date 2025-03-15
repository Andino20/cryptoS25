use bitvec::prelude::*;
use std::fmt::Write;

pub fn encode(data: &[u8]) -> Result<Vec<u8>, &'static str> {
    if data.is_empty() {
        return Err("data is too short");
    }

    let data: BitVec<_, Msb0> = BitVec::from_slice(data);

    let mut code: BitVec<u8, Msb0> = BitVec::new();
    let mut bit_index = 1;
    for bit in &data {
        while is_power_of_2(bit_index) {
            code.push(false);
            bit_index += 1;
        }
        code.push(*bit);
        bit_index += 1;
    }

    let mut parities = 0usize;
    for (index, bit) in code.iter().enumerate() {
        if *bit {
            parities ^= index + 1;
        }
    }
    let parity_count = code.len() - data.len();
    for (index, value) in
        (0..parity_count as u32).map(|x| (2usize.pow(x) - 1, (parities >> x) & 1 == 1))
    {
        code.set(index, value);
    }

    Ok(code.into_vec())
}

pub fn error_correct(code: &[u8]) -> Vec<u8> {
    let mut code: BitVec<u8, Msb0> = BitVec::from_slice(code);
    let error_pos = code.iter().enumerate().fold(0usize, |mut acc, (i, bit)| {
        if *bit {
            acc ^= i + 1;
        }
        acc
    });

    if error_pos > 0 && error_pos - 1 < code.len() {
        let value = *code.get(error_pos - 1).unwrap();
        code.set(error_pos - 1, !value);
    }

    // Remove parity bits (+ 1 is weird and probably not right)
    let parity_count = code.len().ilog2() + 1;
    for i in (0..parity_count).rev() {
        let i = 2usize.pow(i) - 1;
        code.remove(i);
    }

    code.into_vec()
}

pub fn to_hex_string(data: &[u8]) -> String {
    data.iter().fold(String::new(), |mut output, b| {
        let _ = write!(output, "{b:02X}");
        output
    })
}

pub fn to_bit_string(data: &[u8]) -> String {
    data.iter().fold(String::new(), |mut output, &b| {
        (0..8).for_each(|i| {
            let _ = write!(output, "{}", (b >> (7 - i)) & 1);
        });
        let _ = write!(output, " ");
        output
    })
}

fn is_power_of_2(n: usize) -> bool {
    (n > 0) && (n & !(n - 1) == n)
}

#[cfg(test)]
mod tests {
    use crate::hamming::is_power_of_2;

    #[test]
    fn test_power_of_2() {
        assert!(!is_power_of_2(11));
        assert!(!is_power_of_2(31));

        assert!(is_power_of_2(2));
        assert!(is_power_of_2(8));
    }
}
