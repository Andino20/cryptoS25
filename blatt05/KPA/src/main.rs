use rand::{Rng, rng};

fn main() {
    let plain = "World";
    let cipher = black_box(plain);

    let c1 = plain.chars().nth(0);
    let c2 = cipher.chars().nth(0);

    let c1 = c1.unwrap();
    let c2 = c2.unwrap();

    let key = (c2 as i8 - c1 as i8).rem_euclid(26);

    println!("Plain:\t\t{plain}");
    println!("Cipher:\t\t{cipher}");
    println!("Calculated Key:\t{key}");
}

fn black_box(plain: &str) -> String {
    let key = rng().random_range(0..26);
    println!("Secret Key:\t{key}");
    caesar_cipher(plain, key)
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
