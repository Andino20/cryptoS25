use image::RgbaImage;
use std::time::Instant;

mod color;

fn main() -> std::io::Result<()> {
    let image_paths = std::fs::read_dir("../img/cipher")?
        .filter_map(Result::ok)
        .map(|e| e.path());

    for path in image_paths.filter(|p| p.is_file()) {
        let filename = path.file_name().unwrap().to_str().unwrap();
        let start = Instant::now();
        print!("Decrypting {filename}...");
        let cipher_image = image::open(&path).expect("Failed to open image");
        decipher(cipher_image.to_rgba8())
            .save(format!("../img/decipher/{filename}"))
            .unwrap();
        let duration = start.elapsed();
        println!(" done! Took: {}ms", duration.as_millis());
    }

    Ok(())
}

fn decipher(mut img: RgbaImage) -> RgbaImage {
    let (width, height) = img.dimensions();
    let mut pixels = img.as_flat_samples_mut();
    let raw = pixels.as_mut_slice();
    let bpr = (width * 4) as usize;

    for current_row in 0..height - 1 {
        let row1_start = (current_row * width * 4) as usize;
        let (best_row, _) = (current_row + 1..height)
            .map(|r| {
                let row2_start = (r * width * 4) as usize;
                let score = similarity_score(
                    &raw[row1_start..row1_start + bpr],
                    &raw[row2_start..row2_start + bpr],
                );
                (row2_start, score)
            })
            .min_by(|(_, s1), (_, s2)| s1.partial_cmp(s2).unwrap())
            .unwrap();

        for i in 0..(width as usize * 4) {
            raw.swap(((current_row + 1) * width * 4) as usize + i, best_row + i);
        }
    }
    img
}

fn similarity_score(row1: &[u8], row2: &[u8]) -> f32 {
    row1.chunks(4)
        .zip(row2.chunks(4))
        .map(|(chunk1, chunk2)| color::euclidean_distance(chunk1, chunk2))
        .sum::<f32>()
}
