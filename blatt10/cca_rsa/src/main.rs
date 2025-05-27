use num::bigint::ToBigUint;
use num::{BigUint, Integer, One};
use num_prime::{PrimalityTestConfig, RandPrime};

struct PublicKey {
    e: BigUint,
    n: BigUint,
}

struct PrivateKey {
    d: BigUint,
    n: BigUint,
}

impl PrivateKey {
    fn sign(&self, m: &BigUint) -> BigUint {
        m.modpow(&self.d, &self.n)
    }
}

impl PublicKey {
    fn verify(&self, m: &BigUint, signature: &BigUint) -> bool {
        m.eq(&signature.modpow(&self.e, &self.n))
    }
}

fn main() {
    let (public, private) = generate_key_pair();

    let m1 = BigUint::from(90u32);
    let m2 = BigUint::from(12385u32);

    let m3 = &m1 * &m2 % &public.n;

    println!("m1: {}", m1);
    println!("m2: {}", m2);
    println!("m3: {}", m3);

    let m1_sign = private.sign(&m1);
    let m2_sign = private.sign(&m2);

    let m3_sign = m1_sign * m2_sign % &public.n;
    println!("Verify m3 signature: {}", public.verify(&m3, &m3_sign));
}

fn generate_key_pair() -> (PublicKey, PrivateKey) {
    let mut rng = rand::thread_rng();
    let p: BigUint = rng.gen_prime(512, Some(PrimalityTestConfig::strict()));
    let q: BigUint = rng.gen_prime(512, Some(PrimalityTestConfig::strict()));
    let n = &p * &q;

    let phi_n = (p - BigUint::one()) * (q - BigUint::one());

    let e = 65537.to_biguint().unwrap();
    let d = e.modinv(&phi_n).unwrap();

    (PublicKey { e, n: n.clone() }, PrivateKey { d, n })
}
