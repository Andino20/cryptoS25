use image::buffer::Pixels;
use image::{Pixel, Rgba, RgbaImage};
use std::collections::HashSet;

mod color;

fn main() -> std::io::Result<()> {
    /*
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
     */

    /*
    let img = image::open("./img/02_tloztotk.png")
        .expect("failed to open image")
        .to_rgba8();
    decipher(img)
        .save("./img/decipher_02_tloztotk.png")
        .expect("failed to save decipher_02_tloztotk");
     */

    let mut a = vec![3, 2, 1];
    let mut b = vec![9, 8, 7];

    std::mem::swap(a.get_mut(1).unwrap(), b.get_mut(1).unwrap());

    println!("a: {:?}", a);
    println!("b: {:?}", b);

    Ok(())
}

fn decipher(mut img: RgbaImage) -> RgbaImage {
    let mut rows = img.rows_mut().collect::<Vec<_>>();

    for current_row in 0..img.height() - 1 {
        (current_row + 1..img.height())
            .map(|r| similarity(rows.get(current_row).unwrap(), rows.get(r).unwrap()));
    }

    RgbaImage::new(10, 10)
}

fn similarity_score(img: &RgbaImage, row1: u32, row2: u32) -> f32 {
    let width = img.width();

    let mut sum = 0.0;

    for i in 0..width {
        let p1 = img.get_pixel(i, row1).channels();
        let p2 = img.get_pixel(i, row2).channels();

        sum += color::euclidean_distance(p1, p2);
    }
    sum
}

fn similarity(row1: &Pixels<Rgba<u8>>, row2: &Pixels<Rgba<u8>>) -> f32 {

    0.0
}

fn swap_rows(img: &mut RgbaImage, row1: u32, row2: u32) {
    let width = img.width();

    for x in 0..width {
        let p1 = *img.get_pixel(x, row1);
        let p2 = *img.get_pixel(x, row2);
        img.put_pixel(x, row1, p2);
        img.put_pixel(x, row2, p1);
    }
}
