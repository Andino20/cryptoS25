use image::{EncodableLayout, GrayImage, ImageBuffer, Luma};
use std::path::Path;

fn main() {
    let image_folder = Path::new("../images");
    let filename = image_folder.join("img01.jpg");

    let img = image::open(&filename).expect("Failed to open image!");
    let img: GrayImage = img.to_luma8();

    img.save(image_folder.join("out/img01_gray.jpg"))
        .expect("Failed to save image!");

    let mut folded_img = img.clone();
    for i in 0..5 {
        folded_img = baker_fold(folded_img);
        let e = entropy(folded_img.as_bytes());

        println!("Entropy at {}. iteration: {e}", i + 1);
        folded_img
            .save(image_folder.join(format!("out/img_f{i}.bmp")))
            .unwrap();
    }
}

fn baker_fold(imgbuf: ImageBuffer<Luma<u8>, Vec<u8>>) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    assert_eq!(imgbuf.width(), imgbuf.height());
    let size = imgbuf.width();
    let mut buffer: ImageBuffer<Luma<u8>, Vec<u8>> =
        ImageBuffer::new(imgbuf.width(), imgbuf.height());

    for (x, y, color) in imgbuf.enumerate_pixels() {
        let (new_x, new_y) = baker_map_transform(x, y, size);
        buffer.put_pixel(new_x, new_y, *color);
    }

    buffer
}

fn baker_map_transform(x: u32, y: u32, size: u32) -> (u32, u32) {
    let new_x = (x * 2) % size;
    let new_y = if x < size / 2 {
        y / 2
    } else {
        y / 2 + size / 2
    };
    (new_x, new_y)
}

fn entropy(data: &[u8]) -> f32 {
    let mut histogram: [u32; 256] = [0; 256];
    for datum in data {
        histogram[*datum as usize] += 1;
    }

    histogram
        .iter()
        .map(|v| *v as f32 / data.len() as f32)
        .map(|p| -p * p.log2())
        .sum::<f32>()
}
