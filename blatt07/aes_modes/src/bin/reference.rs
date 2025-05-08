use aes::Aes128;
use aes::cipher::KeyInit;
use aes::cipher::block_padding::Pkcs7;
use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use aes_modes::benchmark::time;
use aes_modes::time;
use image::{GrayImage, Luma};
use log::info;

type Aes128CbcEnc = cbc::Encryptor<Aes128>;
type Aes128CbcDec = cbc::Decryptor<Aes128>;

fn main() {
    colog::init();

    let data = [0x42; 64 * 64];
    save(data, "out/reference/original.png");

    let key = [0x42u8; 16]; // 128-bit key
    let iv = [0x24u8; 16]; // 128-bit IV for CBC mode

    let encryptor = Aes128CbcEnc::new(&key.into(), &iv.into());
    let decryptor = Aes128CbcDec::new(&key.into(), &iv.into());

    // --- CBC Mode ---
    let mut buf = vec![0x00; 64 * 64 + 16];
    buf[..data.len()].copy_from_slice(&data);
    let (encrypted_cbc, duration) = time!(
        encryptor
            .encrypt_padded_mut::<Pkcs7>(&mut buf, 64 * 64)
            .unwrap()
    );
    save(&encrypted_cbc[..64 * 64], "out/reference/encrypted-cbc.png");
    info!(
        "REF: Time elapsed encrypting (CBC): {}ns",
        duration.as_micros()
    );

    let (decrypted_cbc, duration) = time!(decryptor.decrypt_padded_mut::<Pkcs7>(&mut buf).unwrap());
    save(decrypted_cbc, "out/reference/decrypted-cbc.png");
    info!(
        "REF: Time elapsed decrypting (CBC): {}ns",
        duration.as_nanos()
    );

    let cipher = Aes128::new(&key.into());

    let mut buf = vec![0x00; 64 * 64 + 16];
    buf[..data.len()].copy_from_slice(&data);
    let (encrypted_ecm, duration) = time!(
        cipher
            .encrypt_padded_mut::<Pkcs7>(&mut buf, 64 * 64)
            .unwrap()
    );
    save(&encrypted_ecm[..64 * 64], "out/reference/encrypted-ecm.png");
    info!(
        "REF: Time elapsed encrypting (ECM): {}ns",
        duration.as_nanos()
    );

    let cipher = Aes128::new(&key.into());
    let (decrypted_ecm, duration) = time!(cipher.decrypt_padded_mut::<Pkcs7>(&mut buf).unwrap());
    save(decrypted_ecm, "out/reference/decrypted-ecm.png");
    info!(
        "REF: Time elapsed decrypting (ECM): {}ns",
        duration.as_micros()
    );
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
