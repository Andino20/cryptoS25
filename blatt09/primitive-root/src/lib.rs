use prime_factorization::Factorization;
use std::collections::HashSet;

pub fn primitive_root_brute_force(p: u128) -> Option<u128> {
    let phi = p - 1;
    for q in 2..phi {
        let mut unique_numbers = HashSet::new();
        let unique_numbers = (1..=phi)
            .take_while(|x| unique_numbers.insert(mod_exp(q, *x, p)))
            .count();

        if unique_numbers == phi as usize {
            return Some(q);
        }
    }
    None
}

pub fn primitive_root_lagrange(p: u128) -> Option<u128> {
    let phi = p - 1;
    let prime_factors = Factorization::run(phi).prime_factor_repr();
    for q in 2..phi {
        let is_primitive_root = prime_factors
            .iter()
            .map(|(factor, _)| mod_exp(q, phi / factor, p))
            .all(|x| x != 1);

        if is_primitive_root {
            return Some(q);
        }
    }
    None
}

fn mod_exp(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brute_force_tests() {
        assert_eq!(primitive_root_brute_force(7), Some(3));
        assert_eq!(primitive_root_brute_force(11), Some(2));
        assert_eq!(primitive_root_brute_force(31223), Some(5));
        assert_eq!(primitive_root_brute_force(32089), Some(13));
        assert_eq!(primitive_root_brute_force(2), None);
    }

    #[test]
    fn lagrange_tests() {
        assert_eq!(primitive_root_lagrange(7), Some(3));
        assert_eq!(primitive_root_lagrange(11), Some(2));
        assert_eq!(primitive_root_lagrange(31223), Some(5));
        assert_eq!(primitive_root_lagrange(32089), Some(13));
        assert_eq!(primitive_root_lagrange(2), None);
    }

    #[test]
    fn exp_mod_tests() {
        assert_eq!(mod_exp(3, 0, 7), 1);
        assert_eq!(mod_exp(3, 1, 7), 3);
        assert_eq!(mod_exp(3, 2, 7), 2);
        assert_eq!(mod_exp(3, 3, 7), 6);
        assert_eq!(mod_exp(3, 4, 7), 4);
        assert_eq!(mod_exp(3, 5, 7), 5);
        assert_eq!(mod_exp(3, 6, 7), 1);
        assert_eq!(mod_exp(5423, 5132, 6317), 3837);
    }
}
