use aes::cipher::block_padding::Pkcs7;
use aes::cipher::KeyInit;
use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use aes::Aes128;
use aes_modes::benchmark::time;
use aes_modes::{generate_aes128_iv, generate_aes128_key, time};
use std::time::Duration;

type Aes128CbcEnc = cbc::Encryptor<Aes128>;
type Aes128CbcDec = cbc::Decryptor<Aes128>;

fn main() {
    colog::init();
    
    // Test different image sizes
    for size in [256, 512, 1024, 2048] {
        benchmark_image_encryption(size, 10);
    }
}

fn benchmark_image_encryption(img_size: usize, iterations: u32) {
    let data = vec![0x42u8; img_size];

    let key = generate_aes128_key();
    let iv = generate_aes128_iv();

    let mut total_encrypt_cbc = Duration::new(0, 0);
    let mut total_decrypt_cbc = Duration::new(0, 0);
    let mut total_encrypt_ecm = Duration::new(0, 0);
    let mut total_decrypt_ecm = Duration::new(0, 0);

    for i in 0..iterations {
        // CBC Mode benchmarking
        let (enc_time_cbc, dec_time_cbc) = benchmark_cbc_mode(&data, &key, &iv, img_size);
        total_encrypt_cbc += enc_time_cbc;
        total_decrypt_cbc += dec_time_cbc;

        // ECM Mode benchmarking
        let (enc_time_ecm, dec_time_ecm) = benchmark_ecm_mode(&data, &key, img_size);
        total_encrypt_ecm += enc_time_ecm;
        total_decrypt_ecm += dec_time_ecm;
    }

    // Calculate and print results for CBC
    print_benchmark_results(
        "CBC", 
        img_size, 
        iterations,
        total_encrypt_cbc,
        total_decrypt_cbc
    );

    // Calculate and print results for ECM
    print_benchmark_results(
        "ECM", 
        img_size, 
        iterations,
        total_encrypt_ecm,
        total_decrypt_ecm
    );
}

fn benchmark_cbc_mode(data: &[u8], key: &[u8; 16], iv: &[u8; 16], img_size: usize) -> (Duration, Duration) {
    let encryptor = Aes128CbcEnc::new(key.into(), iv.into());
    let decryptor = Aes128CbcDec::new(key.into(), iv.into());

    let mut buf = vec![0x00; img_size + 16];
    buf[..data.len()].copy_from_slice(data);

    let (encrypted_cbc, enc_duration) = time!(
        encryptor
            .encrypt_padded_mut::<Pkcs7>(&mut buf, img_size)
            .unwrap()
    );

    let (_, dec_duration) = time!(
        decryptor
            .decrypt_padded_mut::<Pkcs7>(&mut buf)
            .unwrap()
    );

    (enc_duration, dec_duration)
}

fn benchmark_ecm_mode(data: &[u8], key: &[u8; 16], img_size: usize) -> (Duration, Duration) {
    let cipher = Aes128::new(key.into());
    
    let mut buf = vec![0x00; img_size + 16];
    buf[..data.len()].copy_from_slice(data);

    let (_, enc_duration) = time!(
        cipher
            .encrypt_padded_mut::<Pkcs7>(&mut buf, img_size)
            .unwrap()
    );

    let cipher = Aes128::new(key.into());
    let (_, dec_duration) = time!(
        cipher
            .decrypt_padded_mut::<Pkcs7>(&mut buf)
            .unwrap()
    );

    (enc_duration, dec_duration)
}

fn print_benchmark_results(
    mode: &str,
    size: usize,
    iterations: u32,
    total_encrypt: Duration,
    total_decrypt: Duration,
) {
    let avg_encrypt = total_encrypt.as_micros() as f64 / iterations as f64;
    let avg_decrypt = total_decrypt.as_micros() as f64 / iterations as f64;
    let throughput_enc =
        (size as f64 * iterations as f64) / (total_encrypt.as_secs_f64() * 1024.0 * 1024.0);
    let throughput_dec =
        (size as f64 * iterations as f64) / (total_decrypt.as_secs_f64() * 1024.0 * 1024.0);

    println!("=== Benchmark Results ({} mode) ===", mode);
    println!("Input size: {} bytes", size);
    println!("Iterations: {}", iterations);
    println!(
        "Average encryption time: {:.2} µs ({:.2} MB/s)",
        avg_encrypt, throughput_enc
    );
    println!(
        "Average decryption time: {:.2} µs ({:.2} MB/s)",
        avg_decrypt, throughput_dec
    );
}