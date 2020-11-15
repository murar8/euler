pub fn create_wheel(basis: &[u64], limit: u64) -> Vec<u64> {
    let wheel_size = basis.iter().product::<u64>() + 1;

    let spokes = (1..=wheel_size).filter(|n| basis.iter().all(|b| n % b != 0)).collect::<Vec<_>>();

    spokes
        .windows(2)
        .map(|ss| ss[1] - ss[0])
        .cycle()
        .scan(1u64, |acc, v| {
            *acc += v as u64;
            Some(*acc)
        })
        .take_while(|n| *n < limit)
        .collect()
}

pub fn primes_to(limit: usize) -> Vec<u64> {
    let sqrt = (limit as f64).sqrt() as usize;
    let mut candidates = vec![true; limit + 1];

    for i in 2..=sqrt {
        if candidates[i] {
            for j in (i * i..=limit as usize).step_by(i) {
                candidates[j] = false
            }
        }
    }

    candidates.iter().enumerate().skip(2).filter(|(_, p)| **p).map(|(n, _)| n as u64).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_create_wheel() {
        assert_eq!(create_wheel(&[2, 3], 40), [5, 7, 11, 13, 17, 19, 23, 25, 29, 31, 35, 37])
    }

    #[test]
    fn test_primes_to() {
        assert_eq!(
            primes_to(997),
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
    fn bench_wheel(b: &mut Bencher) { b.iter(|| create_wheel(&[2, 3, 5, 7], 10_000_000)); }

    #[bench]
    fn bench_primes_to_1_million(b: &mut Bencher) { b.iter(|| primes_to(1_000_000)); }

    #[bench]
    fn bench_primes_to_10_million(b: &mut Bencher) { b.iter(|| primes_to(10_000_000)); }
}
