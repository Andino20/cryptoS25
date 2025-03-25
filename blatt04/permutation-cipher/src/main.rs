use rand::{Rng, rng};

fn main() -> std::io::Result<()> {
    let image_paths = std::fs::read_dir("./img")?
        .filter_map(Result::ok)
        .map(|e| e.path());

    for path in image_paths.filter(|p| p.is_file()) {
        let filename = path.file_name().unwrap().to_str().unwrap();
        let original = image::open(&path).expect("Failed to open image");

        for block_size in [1, 2, 4, 8, 16, 32] {
            let cipher_image = permutation_cipher(&original, block_size);
            cipher_image
                .save(format!("./img/out/{:02}_{}", block_size, filename))
                .expect("Failed to save image");
        }
    }

    Ok(())
}

fn permutation_cipher(original: &image::DynamicImage, blocks: u32) -> image::DynamicImage {
    let mut img = original.to_rgba8();
    let height = img.height();
    let rows_per_block = height / blocks;

    for block in 0..blocks {
        for _ in 0..1000 {
            let row1 = rng().random_range(0..rows_per_block) + block * rows_per_block;
            let row2 = rng().random_range(0..rows_per_block) + block * rows_per_block;

            if row1 != row2 {
                swap_rows(&mut img, row1, row2);
            }
        }
    }

    image::DynamicImage::ImageRgba8(img)
}

fn swap_rows(img: &mut image::RgbaImage, row1: u32, row2: u32) {
    assert_ne!(row1, row2);
    let width = img.width();

    for x in 0..width {
        let p1 = img.get_pixel(x, row1).clone();
        let p2 = img.get_pixel(x, row2).clone();
        img.put_pixel(x, row1, p2);
        img.put_pixel(x, row2, p1);
    }
}
