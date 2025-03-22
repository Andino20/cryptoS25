use rand::{Rng, rng};
use std::f32::consts::E;

fn main() {
    let english_letter_frequencies: [f32; 26] = [
        0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, 0.06094, 0.06966, 0.00153,
        0.00772, 0.04025, 0.02406, 0.06749, 0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056,
        0.02758, 0.00978, 0.02360, 0.00150, 0.01974, 0.00074,
    ];

    let plain = "World";
    let cipher = caesar_cipher(plain, 20);
    println!("Ciphertext: {cipher}");

    /*
    // Brute Force by visualization and manual checking
    (0..26)
        .map(|shift| (caesar_cipher(&cipher, 26 - shift), shift))
        .for_each(|(s, key)| println!("Key {:02}: {}", key, s));
    */

    let decrypt_attempts = (0..26)
        .map(|shift| caesar_cipher(&cipher, 26 - shift))
        .collect::<Vec<_>>();

    let histograms = decrypt_attempts
        .iter()
        .map(|plain| letter_frequency(plain))
        .collect::<Vec<_>>();

    let mut scores = histograms
        .iter()
        .enumerate()
        .map(|(key, h)| (key, score(&english_letter_frequencies, h)))
        .collect::<Vec<_>>();
    scores.sort_by(|(_, a), (_, b)| b.partial_cmp(&a).unwrap());

    println!("Key\tScore\t\tDecryption Result");
    for (key, score) in scores {
        println!(
            "{:02}\t{:0.5}\t\t{}",
            key,
            score,
            caesar_cipher(&cipher, 26 - key)
        );
    }
}

fn caesar_cipher(text: &str, key: usize) -> String {
    let key = (key % 26) as u8;
    text.chars()
        .map(|c| shift_letter(c, key))
        .fold(String::new(), |mut out, c| {
            out.push(c);
            out
        })
}

fn shift_letter(c: char, shift: u8) -> char {
    match c {
        'A'..='Z' => ((c as u8 - b'A' + shift) % 26 + b'A') as char,
        'a'..='z' => ((c as u8 - b'a' + shift) % 26 + b'a') as char,
        _ => c,
    }
}

fn letter_frequency(text: &str) -> [f32; 26] {
    let mut histogram: [f32; 26] = [0f32; 26];
    for c in text.chars() {
        match c {
            'A'..='Z' => histogram[(c as u8 - b'A') as usize] += 1.0,
            'a'..='z' => histogram[(c as u8 - b'a') as usize] += 1.0,
            _ => {}
        }
    }

    let letter_count = histogram.iter().sum::<f32>();
    for frequency in histogram.as_mut() {
        *frequency /= letter_count;
    }

    histogram
}

fn score(reference: &[f32; 26], h: &[f32; 26]) -> f32 {
    E.powf(
        -reference
            .iter()
            .zip(h)
            .map(|(&x, &y)| (x - y).abs())
            .sum::<f32>(),
    )
}
