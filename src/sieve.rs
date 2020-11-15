use std::{cmp::Ordering, collections::BinaryHeap};

use crate::wheel::Wheel;

#[derive(Eq, PartialEq)]

struct Multiple {
    base: u64,
    value: u64,
}

impl Ord for Multiple {
    fn cmp(&self, other: &Self) -> Ordering { other.value.cmp(&self.value) }
}

impl PartialOrd for Multiple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

pub struct Sieve {
    multiples: BinaryHeap<Multiple>,
    wheel: Wheel,
}

impl Sieve {
    pub fn new() -> Self {
        let basis = vec![2, 3, 5, 7];
        Sieve { multiples: BinaryHeap::new(), wheel: Wheel::new(basis) }
    }
}

impl Iterator for Sieve {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let candidate = self.wheel.next().unwrap();

        while self.multiples.peek().map_or(false, |m| m.value < candidate) {
            let Multiple { base, value: _ } = self.multiples.pop().unwrap();
            let value = base * (1 + ((candidate - 1) / base));
            self.multiples.push(Multiple { base, value });
        }

        if self.multiples.peek().map_or(false, |m| m.value == candidate) {
            return self.next();
        }

        if candidate > *self.wheel.basis.last().unwrap() {
            let multiple = Multiple { base: candidate, value: candidate * candidate };
            self.multiples.push(multiple);
        }

        Some(candidate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_multiple() {
        assert_eq!(
            Multiple { base: 2, value: 4 }.cmp(&Multiple { base: 4, value: 4 }),
            Ordering::Equal
        );

        assert_eq!(
            Multiple { base: 2, value: 4 }.cmp(&Multiple { base: 2, value: 8 }),
            Ordering::Greater
        );

        assert_eq!(
            Multiple { base: 2, value: 8 }.cmp(&Multiple { base: 4, value: 4 }),
            Ordering::Less
        );
    }

    #[test]
    fn test_primes_to() {
        let sieve = Sieve::new();

        assert_eq!(
            sieve.take_while(|n| *n < 1000).collect::<Vec<_>>(),
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
                997
            ]
        );
    }

    #[bench]
    fn bench_sieve_10k(b: &mut Bencher) {
        b.iter(|| Sieve::new().take_while(|n| *n < 10_000).for_each(drop))
    }

    #[bench]
    fn bench_sieve_100k(b: &mut Bencher) {
        b.iter(|| Sieve::new().take_while(|n| *n < 100_000).for_each(drop))
    }
}
