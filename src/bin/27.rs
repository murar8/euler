use euler::{eratosthenes::primes_to, sieve::Sieve};

fn main() {
    let sieve = Sieve::new();
    let time = std::time::Instant::now();
    let _ = sieve.take_while(|n| *n < 100_000).for_each(|_| {});
    println!("time for iterator: {}", std::time::Instant::now().duration_since(time).as_micros());

    let time = std::time::Instant::now();
    let _ = primes_to(100_000_000);
    println!("time for sieve: {}", std::time::Instant::now().duration_since(time).as_micros());
}
