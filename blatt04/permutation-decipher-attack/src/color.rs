pub fn euclidean_distance(color1: &[u8], color2: &[u8]) -> f32 {
    assert_eq!(color1.len(), color2.len());
    color1
        .iter()
        .zip(color2.iter())
        .map(|(c1, c2)| (*c1 as i32, *c2 as i32))
        .map(|(c1, c2)| ((c1 - c2) * (c1 - c2)) as f32)
        .sum::<f32>()
        .sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_distance() {
        assert_eq!(euclidean_distance(&[0, 64, 0], &[255, 64, 0]), 255.0);
    }
}
