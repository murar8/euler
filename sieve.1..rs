use crate::wheel::{self, Basis, Wheel};
use std::collections::BTreeSet;

use std::f64;

const WHEEL_BASIS: Basis = wheel::BASIS_BIG;

pub struct Sieve {
    primes: BTreeSet<u64>,
    wheel: Wheel,
    wheel_basis: Vec<u64>,
}

impl Sieve {
    pub fn new() -> Self {
        Sieve {
            primes: BTreeSet::new(),
            wheel: Wheel::new(WHEEL_BASIS),
            wheel_basis: WHEEL_BASIS.iter().copied().rev().collect::<Vec<_>>(),
        }
    }
}

impl Iterator for Sieve {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.wheel_basis.is_empty() {
            return self.wheel_basis.pop();
        }

        let candidate = self.wheel.next().unwrap();
        let sqrt = (candidate as f64).sqrt() as u64;

        let is_prime = self.primes.range(0..=sqrt).all(|p| candidate % p != 0);

        if is_prime {
            self.primes.insert(candidate);
            return Some(candidate);
        }

        self.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_primes_to() {
        let sieve = Sieve::new();

        assert_eq!(
            sieve.take_while(|n| *n < 1088).collect::<Vec<_>>(),
            [
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167,
                173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257,
                263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353,
                359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449,
                457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563,
                569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653,
                659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761,
                769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877,
                881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991,
                997, 1009, 1013, 1019, 1021, 1031, 1033, 1039, 1049, 1051, 1061, 1063, 1069, 1087
            ]
        );
    }

    #[bench]
    fn bench_sieve(b: &mut Bencher) {
        b.iter(|| {
            let sieve = Sieve::new();
            sieve.take(10000).for_each(drop);
        });
    }
}
