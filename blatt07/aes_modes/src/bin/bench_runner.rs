use aes_modes::benchmark::time;
use aes_modes::{BlockCipherMode, decrypt, encrypt, generate_aes128_iv, generate_aes128_key, time};
use std::time::Duration;

fn main() {
    for size in [256, 512, 1024, 2048] {
        benchmark_cipher(size, 10, &generate_aes128_key(), BlockCipherMode::ECM);
    }

    for size in [256, 512, 1024, 2048] {
        benchmark_cipher(
            size,
            10,
            &generate_aes128_key(),
            BlockCipherMode::CBC(generate_aes128_iv()),
        );
    }
}

fn benchmark_cipher(size: usize, iterations: u32, key: &[u8; 16], mode: BlockCipherMode) {
    // Create random input data of specified size
    let input: Vec<u8> = (0..size).map(|_| rand::random::<u8>()).collect();

    let mut total_encrypt = Duration::new(0, 0);
    let mut total_decrypt = Duration::new(0, 0);

    for i in 0..iterations {
        // Benchmark encryption
        let (encrypted, enc_time) = time!(encrypt(&input, key, mode.clone()));
        total_encrypt += enc_time;

        // Benchmark decryption
        let (decrypted, dec_time) = time!(decrypt(&encrypted, key, mode.clone()));
        total_decrypt += dec_time;

        // Verify correctness
        assert_eq!(
            input, decrypted,
            "Decryption failed to recover original data"
        );
    }

    let avg_encrypt = total_encrypt.as_micros() as f64 / iterations as f64;
    let avg_decrypt = total_decrypt.as_micros() as f64 / iterations as f64;
    let throughput_enc =
        (size as f64 * iterations as f64) / (total_encrypt.as_secs_f64() * 1024.0 * 1024.0);
    let throughput_dec =
        (size as f64 * iterations as f64) / (total_decrypt.as_secs_f64() * 1024.0 * 1024.0);

    println!("=== Benchmark Results ({} mode) ===", mode.short_name());
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
