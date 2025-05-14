use aes_modes::*;
use image::{GrayImage, Luma};
use log::info;
use rand::{Rng, rng};

const IMG_SIZE: usize = 64 * 64;

fn main() {
    colog::init();
    run_experiment(BlockCipherMode::ECM);
    run_experiment(BlockCipherMode::CBC(generate_aes128_iv()));
}

fn run_experiment(m: BlockCipherMode) {
    info!("Running experiment with {m}");

    let key = generate_aes128_key();
    let msg = [0x42; IMG_SIZE];
    save(msg, "out/original.png");

    let mut cipher = encrypt(msg, &key, m.clone());
    random_bit_error(&mut cipher);
    let plain = decrypt(&cipher, &key, m.clone());

    save(&cipher, &format!("out/output_{}.png", m.short_name()));
    save(
        &plain,
        &format!("out/output_{}_decrypt.png", m.short_name()),
    );
}

fn save<T: AsRef<[u8]>>(data: T, filename: &str) {
    let data = data.as_ref();
    let width = 64;
    let height = 64;
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
