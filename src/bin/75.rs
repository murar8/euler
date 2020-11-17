use std::collections::BinaryHeap;

#[derive(Debug, Eq, PartialEq)]
struct Triple(u64, u64, u64);

impl PartialOrd for Triple {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((self.0 + self.1 + self.2).cmp(&(other.0 + other.1 + other.2)))
    }
}

impl Ord for Triple {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.partial_cmp(other).unwrap() }
}

#[derive(Debug, Eq, PartialEq)]
struct CoprimePair(u64, u64);

impl PartialOrd for CoprimePair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let triple = |m, n| (m * m - n * n) + (2 * m * n) + (m * m + n * n);
        Some(triple(other.0, other.1).cmp(&triple(self.0, self.1)))
    }
}

impl Ord for CoprimePair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.partial_cmp(other).unwrap() }
}

#[derive(Debug)]
struct TripleGenerator {
    coprimes: BinaryHeap<CoprimePair>,
}

impl TripleGenerator {
    fn new() -> Self {
        TripleGenerator { coprimes: BinaryHeap::from(vec![CoprimePair(2, 1), CoprimePair(3, 1)]) }
    }
}

impl Iterator for TripleGenerator {
    type Item = Triple;

    fn next(&mut self) -> Option<Self::Item> {
        let CoprimePair(m, n) = self.coprimes.pop().unwrap();

        self.coprimes.push(CoprimePair(2 * m - n, m));
        self.coprimes.push(CoprimePair(2 * m + n, m));
        self.coprimes.push(CoprimePair(2 * n + m, n));

        // println!("{:?}", Triple(m * m - n * n, 2 * m * n, m * m + n * n));
        Some(Triple(m * m - n * n, 2 * m * n, m * m + n * n))
    }
}

fn main() {
    println!("{:?}", 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triple_generator() {
        let tg = TripleGenerator::new();
        assert_eq!(tg.take(1000).map(|k| k.0 + k.1 + k.2).collect::<Vec<_>>(), []);
    }
}
