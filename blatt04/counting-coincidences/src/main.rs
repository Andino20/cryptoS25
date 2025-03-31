use rand::{Rng, rng};

fn main() {
    let plain = "Wird nicht ein kurzer repetitiver Schluessel verwendet sondern einer der
die gleiche Laenge aufweist wie der Plaintext, spricht man von OTP
Verschluesselung";
    //let plain = &"X".repeat(35);
    let key = generate_key(2);
    let cipher = short_key_xor(plain, &key);

    println!("Plaintext ({}):\t{}", plain.len(), plain);
    println!("Key ({}):\t{:?}", key.len(), key);
    println!("Ciphertext:\t{}", cipher.len());

    let key_length = counting_coincidences(&cipher);
    println!("Expected Key length:\t{}", key_length);
}

fn generate_key(length: usize) -> Vec<u8> {
    rng().random_iter().take(length).collect::<Vec<u8>>()
}

fn short_key_xor(text: &str, key: &[u8]) -> Vec<u8> {
    let padding = key.len() - text.len() % key.len();
    text.chars()
        .chain(std::iter::repeat_n('X', padding))
        .enumerate()
        .map(|(i, c)| c as u8 ^ key[i % key.len()])
        .collect()
}

fn counting_coincidences(text: &[u8]) -> usize {
    println!("SHIFT\tIOC");
    for offset in 1..text.len() {
        let same_bytes = text
            .iter()
            .zip(text.iter().chain(text).skip(offset))
            .filter(|&(a, b)| a == b)
            .count() as f32
            / text.len() as f32;
        println!("{:02}\t{:.2}%", offset, same_bytes * 100.0);
        if same_bytes > 0.06 {
            return offset;
        }
    }
    text.len()
}
