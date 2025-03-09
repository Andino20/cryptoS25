use aes::cipher::KeyIvInit;
use cbc::cipher::BlockEncryptMut;
use cbc::cipher::block_padding::Pkcs7;
use image::GrayImage;
use std::fs::DirEntry;
use std::time::*;
mod encryption;
#[derive(Default)]
struct EncryptionStatistics {
    duration: Duration,
    entropy_before: f32,
    entropy_after: f32,
}

fn main() -> std::io::Result<()> {
    let image_paths = std::fs::read_dir("./images")?;
    let stats = image_paths
        .filter_map(Result::ok)
        .map(|entry| encrypt_image(&entry))
        .collect::<Vec<EncryptionStatistics>>();

    let average_entropy_before =
        stats.iter().map(|stats| stats.entropy_before).sum::<f32>() / (stats.len() as f32);
    let average_entropy_after =
        stats.iter().map(|stats| stats.entropy_after).sum::<f32>() / (stats.len() as f32);
    let average_time = stats
        .iter()
        .map(|stats| stats.duration)
        .sum::<Duration>()
        .as_micros() as f32
        / (stats.len() as f32)
        / 1000.0;

    println!("============================ STATS ============================");
    println!("Average entropy (original):\t{}", average_entropy_before);
    println!("Average entropy (encrypted):\t{}", average_entropy_after);
    println!("Average encryption time:\t{} ms", average_time);
    Ok(())
}

fn encrypt_image(file: &DirEntry) -> EncryptionStatistics {
    let filename = file.file_name();
    let mut stats: EncryptionStatistics = Default::default();

    // Load image from memory as grayscale
    let image = image::open(file.path()).unwrap().to_luma8();
    image
        .save(format!("./out/gray_{}", filename.to_str().unwrap()))
        .expect("Could not save image in directory './out'");
    stats.entropy_before = entropy(&image);

    let image_width = image.width();
    let image_height = image.height();

    // Initialize AES encryptor
    let key = encryption::generate_aes128_key();
    let iv = encryption::generate_aes128_iv();
    let cipher = encryption::Aes128CbcEnc::new(&key.into(), &iv.into());

    // Encrypt image data and time the process
    let data = image.into_raw();
    let start = Instant::now();
    let encrypted_data = cipher.encrypt_padded_vec_mut::<Pkcs7>(&data);
    stats.duration = start.elapsed();

    // Save encrypted image to output directory
    let encrypted_image = GrayImage::from_raw(image_width, image_height, encrypted_data)
        .expect("Failed to convert encrypted data into image!");
    encrypted_image
        .save(format!("./out/encrypted_{}", filename.to_str().unwrap()))
        .expect("Could not save encrypted image in directory './out'");
    stats.entropy_after = entropy(&encrypted_image);

    println!("===========================================================");
    println!("Stats of {}", filename.to_str().unwrap());
    println!("Entropy original:\t{}", stats.entropy_before);
    println!("Entropy encrypted:\t{}", stats.entropy_after);

    stats
}

fn entropy(img: &GrayImage) -> f32 {
    let mut histogram: [u32; 256] = [0; 256];
    for value in img.as_raw() {
        histogram[*value as usize] += 1;
    }
    histogram
        .iter()
        .filter(|&&x| x > 0)
        .map(|&x| x as f32 / (img.width() * img.height()) as f32)
        .map(|p| -p * p.log2())
        .sum::<f32>()
}
