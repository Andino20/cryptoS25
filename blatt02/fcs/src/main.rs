use fcs::hamming;

fn main() {
    let key: Vec<u8> = vec![0x42; 16];
    println!("{}", hamming::to_hex_string(&key));
}
