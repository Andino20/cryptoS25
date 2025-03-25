use image::RgbaImage;
use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let image_paths = std::fs::read_dir("../img/cipher")?
        .filter_map(Result::ok)
        .map(|e| e.path());

    for path in image_paths.filter(|p| p.is_file()) {
        let filename = path.file_name().unwrap().to_str().unwrap();
        let cipher_image = image::open(&path).expect("Failed to open image");
        decipher(cipher_image.to_rgba8())
            .save(format!("../img/decipher/{filename}"))
            .unwrap();
    }

    Ok(())
}

fn decipher(img: RgbaImage) -> RgbaImage {
    let rows: HashSet<u32> = HashSet::from_iter(1..img.height());
    
    img
}

fn _swap_rows(img: &mut RgbaImage, row1: u32, row2: u32) {
    assert_ne!(row1, row2);
    let width = img.width();

    for x in 0..width {
        let p1 = *img.get_pixel(x, row1);
        let p2 = *img.get_pixel(x, row2);
        img.put_pixel(x, row1, p2);
        img.put_pixel(x, row2, p1);
    }
}
