use rand::{Rng, rng};

fn main() {
    let plain = "Wird nicht ein kurzer repetitiver Schluessel verwendet sondern einer der
die gleiche Laenge aufweist wie der Plaintext, spricht man von OTP
Verschluesselung";
    let key = generate_key(12);
    let cipher = short_key_xor(plain, &key);

    println!("Plaintext ({}):\t{}", plain.len(), plain);
    println!("Key ({}):\t{:?}", key.len(), key);
    println!("Ciphertext:\t{}\n", cipher.len());

    let key_length = best_key_guess(&cipher);
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

fn best_key_guess(cipher: &[u8]) -> usize {
    let distances = (1..cipher.len() / 2)
        .map(|key_length|
            (key_length, average_hamming_distance(cipher, key_length) / key_length as f32)
        )
        .collect::<Vec<_>>();

    println!("KEY\tAVG_DISTANCE");
    for (key, avg_distance) in distances {
        println!("{key}\t{:.3}", avg_distance / key as f32);
    }
    0
}

fn average_hamming_distance(cipher: &[u8], block_size: usize) -> f32 {
    let cipher_blocks = cipher.chunks(block_size).collect::<Vec<_>>();

    let mut distances = Vec::new();
    for block_pair in cipher_blocks.windows(2) {
        let a = block_pair[0];
        let b = block_pair[1];

        distances.push(hamming_distance(a, b) as f32);
    }

    distances.iter().sum() / distances.len() as f32
}

fn hamming_distance(a: &[u8], b: &[u8]) -> u32 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x ^ y).count_ones())
        .sum::<u32>()
}
