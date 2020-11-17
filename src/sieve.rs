use std::cmp::Reverse;

use priority_queue::PriorityQueue;

use crate::wheel::Wheel;

pub struct Sieve {
    multiples: PriorityQueue<u64, Reverse<u64>>,
    wheel: Wheel,
    candidate: u64,
}

impl Sieve {
    pub fn new() -> Self {
        let basis = vec![2, 3, 5, 7];
        Sieve {
            candidate: *basis.last().unwrap(),
            multiples: PriorityQueue::new(),
            wheel: Wheel::new(basis),
        }
    }

    fn advance(&mut self) {
        self.candidate = self.wheel.next().unwrap();

        while let Some((&base, &Reverse(value))) = self.multiples.peek() {
            if value == self.candidate {
                return self.advance();
            }

            if value > self.candidate {
                break;
            }

            let value = base * (1 + ((self.candidate - 1) / base));
            self.multiples.change_priority(&base, Reverse(value));
        }

        if self.candidate > *self.wheel.basis.last().unwrap() {
            self.multiples.push(self.candidate, Reverse(self.candidate * self.candidate));
        }
    }

    fn advance_to(&mut self, n: u64) {
        while self.candidate < n {
            self.advance();
        }
    }

    pub fn is_prime(&mut self, n: u64) -> bool {
        self.advance_to(n);
        self.wheel.basis.contains(&n) || self.multiples.get(&n).is_some()
    }

    pub fn primes_to(&mut self, limit: u64) -> Vec<u64> {
        self.advance_to(limit);
        self.wheel
            .basis
            .iter()
            .copied()
            .chain(self.multiples.iter().map(|(b, _)| *b))
            .take_while(|n| *n < limit) // TODO drop last?
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const PRIMES_1000: [u64; 168] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
        283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397,
        401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503,
        509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619,
        631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743,
        751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863,
        877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997,
    ];

    #[test]
    fn test_primes_to() {
        let mut sieve = Sieve::new();
        assert_eq!(sieve.primes_to(1000), PRIMES_1000);
    }

    #[test]
    fn test_is_prime() {
        let mut sieve = Sieve::new();
        assert_eq!((1..1000).filter(|n| sieve.is_prime(*n)).collect::<Vec<_>>(), PRIMES_1000);
    }

    #[bench]
    fn bench_primes_to_10k(b: &mut Bencher) { b.iter(|| Sieve::new().primes_to(10_000)) }

    #[bench]
    fn bench_primes_to_100k(b: &mut Bencher) { b.iter(|| Sieve::new().primes_to(100_000)) }

    #[bench]
    fn bench_is_prime_100k(b: &mut Bencher) {
        b.iter(|| {
            let mut sieve = Sieve::new();
            (1..100000).filter(|n| sieve.is_prime(*n)).for_each(drop)
        })
    }
}
