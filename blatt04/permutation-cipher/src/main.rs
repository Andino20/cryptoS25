use rand::{Rng, rng};
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let image_paths = std::fs::read_dir("../img")?
        .filter_map(Result::ok)
        .map(|e| e.path());

    for path in image_paths.filter(|p| p.is_file()) {
        let filename = path.file_name().unwrap().to_str().unwrap();
        let original = image::open(&path).expect("Failed to open image");

        for blocks in [1, 4, 8, 16, 32] {
            print!("Encrypting {filename} with {blocks} Blocks...");
            let start = Instant::now();
            let cipher_image = permutation_cipher(&original, blocks);
            cipher_image
                .save(format!("../img/cipher/{:02}_{}", blocks, filename))
                .expect("Failed to save image");
            let duration = start.elapsed();
            println!(
                " done! Took {}ms. Saved to ../img/cipher/{:02}_{}",
                duration.as_millis(),
                blocks,
                filename
            );
        }
    }

    Ok(())
}

fn permutation_cipher(original: &image::DynamicImage, blocks: u32) -> image::DynamicImage {
    let mut img = original.to_rgba8();
    let rows_per_block = img.height() / blocks;

    for block in 0..blocks {
        for _ in 0..1000 {
            let row1 = rng().random_range(0..rows_per_block) + block * rows_per_block;
            let row2 = rng().random_range(0..rows_per_block) + block * rows_per_block;

            swap_rows(&mut img, row1, row2);
        }
    }

    image::DynamicImage::ImageRgba8(img)
}

fn swap_rows(img: &mut image::RgbaImage, row1: u32, row2: u32) {
    if row1 != row2 {
        let width = img.width();

        for x in 0..width {
            let p1 = img.get_pixel(x, row1).clone();
            let p2 = img.get_pixel(x, row2).clone();
            img.put_pixel(x, row1, p2);
            img.put_pixel(x, row2, p1);
        }
    }
}
