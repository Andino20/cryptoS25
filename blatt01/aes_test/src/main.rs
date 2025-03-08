use aes::cipher::BlockEncryptMut;
use aes::{Aes128, cipher::KeyIvInit, cipher::block_padding::Pkcs7};
use cbc::cipher::BlockDecryptMut;
use image::GrayImage;
use rand::Rng;
use std::env;
use std::process::exit;

type Aes128CbcEnc = cbc::Encryptor<Aes128>;
type Aes128CbcDec = cbc::Decryptor<Aes128>;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please specify a filename");
        exit(1);
    }

    let img = image::open(&args[1])
        .expect("Failed to open image")
        .to_luma8();
    img.save("out/img/gray.png").expect("Failed to save img");

    let data = img.into_raw();
    println!("Entropy of original image: {}", entropy(&data));

    assert_eq!(data.len() % 16, 0);

    let key: [u8; 16] = [30; 16];
    let iv = random_iv();
    let cipher = Aes128CbcEnc::new(&key.into(), &iv.into());
    let encrypted_data = cipher.encrypt_padded_vec_mut::<Pkcs7>(&data);

    let encrypted_image = GrayImage::from_raw(1024, 1024, encrypted_data)
        .expect("Failed to convert encrypted data into image.");
    encrypted_image
        .save("out/img/enc.png")
        .expect("Failed to save enc.png");

    let data = encrypted_image.into_raw();
    println!("Entropy of encrypted image: {}", entropy(&data));

    let decryptor = Aes128CbcDec::new(&key.into(), &iv.into());
    let decrypted_data = decryptor
        .decrypt_padded_vec_mut::<Pkcs7>(&data)
        .expect("Failed to decrypt data");

    let decrypted_image = GrayImage::from_raw(1024, 1024, decrypted_data)
        .expect("Failed to convert encrypted data into image.");
    decrypted_image
        .save("out/img/dec.png")
        .expect("Failed to save enc.png");
}

fn random_iv() -> [u8; 16] {
    let mut iv = [0; 16];
    rand::rng().fill(&mut iv);
    iv
}

fn entropy(data: &[u8]) -> f32 {
    let mut histogram: [u32; 256] = [0; 256];
    for datum in data {
        histogram[*datum as usize] += 1;
    }
    histogram
        .iter()
        .map(|x| *x as f32 / data.len() as f32)
        .map(|p| -p * p.log2())
        .sum::<f32>()
}
