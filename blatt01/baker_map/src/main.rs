use image::GrayImage;
use rand::Rng;
use std::path::Path;

fn main() {
    let mut rng = rand::rng();
    let image_folder = Path::new("../images");
    let filename = image_folder.join("img01.jpg");

    let img = image::open(&filename).expect("Failed to open image!");
    let img: GrayImage = img.to_luma8();

    let k = 5;
    let n: Vec<u32> = vec![k];



    println!("N: {:?}", n);
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
