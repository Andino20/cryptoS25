use rand::{Rng, rng};

fn main() {
    let plain = "Methode von Kasiski: Identische sich wiederholdende Teile des Plaintexts ge-XORed mit dem gleichen Teil des Keys ergeben sich
wiederholende identische Ciphertext Teile. Die Groesse des Shifts
zwischen dem Beginn solcher sich wiederholenden Teile im
Ciphertext sollte daher ein Vielfaches der Key-Laenge sein. Die
Analyse der gemeinsamen Faktoren dieser Shifts identifiziert den
haeufigsten Faktor der dann der Key-laenge entspricht.";
    let key = generate_key(7);
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
            .map(|(&a, &b)| a ^ b == 0)
            .filter(|&b| b)
            .count() as f32
            / text.len() as f32;
        println!("{:02}\t{:.2}%", offset, same_bytes * 100.0);
        if same_bytes > 0.06 {
            return offset;
        }
    }
    text.len()
}
