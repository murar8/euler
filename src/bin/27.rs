use euler::sieve::Sieve;

fn main() {
    let mut sieve = Sieve::new();

    let mut max = (0, 0, 0, 0);

    for a in -999i64..=999 {
        for b in -1000i64..=1000 {
            for n in 0.. {
                let quadratic = (n * n) + (a * n) + b;
                if quadratic >= 0 && sieve.is_prime(quadratic as u64) {
                    if n > max.1 {
                        max = (quadratic, n, a, b);
                    }
                } else {
                    break;
                }
            }
        }
    }

    println!("{:?}", max.2 * max.3);
}
