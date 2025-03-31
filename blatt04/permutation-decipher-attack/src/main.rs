use image::{Pixel, Rgba, RgbaImage};
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
        decipher(&cipher_image.to_rgba8())
            .save(format!("../img/decipher/{filename}"))
            .unwrap();
        let duration = start.elapsed();
        println!(" done! Took: {}ms", duration.as_millis());
    }

    Ok(())
}

fn decipher(img: &RgbaImage) -> RgbaImage {
    let mut rows = img
        .rows()
        .map(|pixels| pixels.collect::<_>())
        .collect::<Vec<_>>();

    for i in 0..img.height() as usize - 1 {
        if let Some(best_match) = nearest_neighbour(i, &rows) {
            rows.swap(i + 1, best_match);
        }
    }

    RgbaImage::from_fn(img.width(), img.height(), |x, y| {
        *rows[y as usize][x as usize]
    })
}

fn nearest_neighbour(start: usize, rows: &[Vec<&Rgba<u8>>]) -> Option<usize> {
    let (result, _) = (start + 1..rows.len())
        .map(|r| (r, similarity(&rows[r], &rows[start])))
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())?;
    Some(result)
}

fn similarity(r1: &[&Rgba<u8>], r2: &[&Rgba<u8>]) -> f32 {
    r1.iter()
        .zip(r2.iter())
        .map(|(&a, &b)| color::euclidean_distance(a.channels(), b.channels()))
        .sum()
}
