pub fn encode(data: &[u8]) -> Result<Vec<u8>, &'static str> {
    if data.len() < 1 {
        return Err("data is too short");
    }

    let data_bits = data.len() as u32 * 8;
    let parities = parity_count(data_bits);

    let mut byte_counter = 0;
    for i in 1..=(data_bits + parities) {}

    Ok(vec![])
}

pub fn parity_count(data_bits: u32) -> u32 {
    let r = data_bits.ilog2();
    if 2u32.pow(r) < data_bits + r + 1 {
        return r + 1;
    }
    r
}

pub fn error_correct(_code: &[u8]) -> () {}

pub fn to_hex_string(data: &[u8]) -> String {
    data.iter().map(|&x| format!("{:02X}", x)).collect::<_>()
}

fn is_power_of_2(n: u32) -> bool {
    (n > 0) && (n & !(n - 1) == n)
}

#[cfg(test)]
mod tests {
    use crate::hamming::parity_count;

    #[test]
    fn test_parity_count() {
        assert_eq!(parity_count(11), 4);
        assert_eq!(parity_count(128), 8);
        assert_eq!(parity_count(256), 9);
    }
}
