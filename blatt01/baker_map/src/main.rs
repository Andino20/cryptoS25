
fn main() {
    let n: Vec<u32> = vec![0, 128, 192, 448, 512];
    println!("N: {:?}", n.iter().sum::<u32>());
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
