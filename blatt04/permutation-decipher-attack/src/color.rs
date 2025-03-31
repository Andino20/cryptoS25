pub fn euclidean_distance(color1: &[u8], color2: &[u8]) -> f32 {
    assert_eq!(color1.len(), color2.len());
    let diff = color1
        .iter()
        .zip(color2.iter())
        .map(|(&x, &y)| (x as i32 - y as i32).abs())
        .sum::<i32>();

    diff as f32 / (color1.len() * 4 * 255) as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_distance() {
        assert_eq!(euclidean_distance(&[0, 64, 0], &[255, 64, 0]), 255.0);
    }
}
