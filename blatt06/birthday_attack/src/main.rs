use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let mut a = fs::read("./documents/mietvertrag_original.pdf")?;
    let mut b = fs::read("./documents/mietvertrag_changed.pdf")?;

    let mut a_map: HashMap<[u8; 4], u32> = HashMap::new();
    let mut b_map: HashMap<[u8; 4], u32> = HashMap::new();

    let start = Instant::now();
    for i in 0..u32::MAX {
        a[11..11 + 4].copy_from_slice(&i.to_be_bytes());
        b[11..11 + 4].copy_from_slice(&i.to_be_bytes());

        let a_digest = short_md5(&a);
        let b_digest = short_md5(&b);

        if let Some(modification) = a_map.get(&b_digest) {
            println!("Found Collision! {:?}", modification);
            a[11..11 + 4].copy_from_slice(&modification.to_be_bytes());
            break;
        }

        if let Some(modification) = b_map.get(&a_digest) {
            println!("Found Collision! {:?}", modification);
            b[11..11 + 4].copy_from_slice(&modification.to_be_bytes());
            break;
        }

        (i % 1000 == 0).then(|| println!("Trying {i}..."));
        a_map.insert(a_digest, i);
        b_map.insert(b_digest, i);
    }
    let duration = start.elapsed();

    println!("Original:\t{:?}", md5::compute(&a));
    println!("Changed:\t{:?}", md5::compute(&b));
    println!("Duration:\t{:?}ms", duration.as_millis());

    fs::write("./documents/mietvertrag_original_adjusted.pdf", a)?;
    fs::write("./documents/mietvertrag_changed_adjusted.pdf", b)?;
    Ok(())
}

fn short_md5<T: AsRef<[u8]>>(data: T) -> [u8; 4] {
    let digest = md5::compute(data).0;
    digest[..4].try_into().unwrap()
}
