use aes_modes::*;
use image::{GrayImage, Luma};
use rand::{Rng, rng};

fn main() {
    colog::init();

    let key = generate_aes128_key();
    let msg = [0x42; 64 * 64];
    save(msg, "out/original.png");

    let cipher = encrypt(msg, &key, BlockCipherMode::ECM);
    save(&cipher, "out/output_ecm.png");

    let iv = generate_aes128_iv();
    let mut cipher = encrypt(msg, &key, BlockCipherMode::CBC(iv.clone()));
    save(&cipher, "out/output_cbc.png");

    random_bit_error(&mut cipher);

    let decipher = decrypt(cipher, &key, BlockCipherMode::CBC(iv.clone()));
    save(&decipher, "out/output_cbc_decrypt.png");
}

fn save<T: AsRef<[u8]>>(data: T, filename: &str) {
    let data = data.as_ref();
    let width = 64;
    let height = data.len().div_ceil(64);
    let mut img = GrayImage::new(width as u32, height as u32);
    for (i, pixel) in data.iter().enumerate() {
        let x = (i % width) as u32;
        let y = (i / width) as u32;
        img.put_pixel(x, y, Luma([*pixel]));
    }

    img.save(filename).unwrap();
}

fn random_bit_error(data: &mut [u8]) {
    let byte = rng().random_range(..data.len());
    let bit: usize = rng().random_range(..8);
    data[byte] ^= 1 << bit;
}
