use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Eq, PartialEq)]
struct Triple {
    k: u64,
    m: u64,
    n: u64,
}

impl Triple {
    fn a(&self) -> u64 { self.k * (self.m * self.m - self.n * self.n) }

    fn b(&self) -> u64 { self.k * (2 * self.m * self.n) }

    fn c(&self) -> u64 { self.k * (self.m * self.m + self.n * self.n) }

    fn perimeter(&self) -> u64 { self.a() + self.b() + self.c() }
}

impl Ord for Triple {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { other.perimeter().cmp(&self.perimeter()) }
}

impl PartialOrd for Triple {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(&other)) }
}

#[derive(Debug)]
struct TripleGenerator {
    max_perimeter: u64,
    triples: BinaryHeap<Triple>,
}

impl TripleGenerator {
    fn new(max_perimeter: u64) -> Self {
        let roots = vec![Triple { k: 1, m: 2, n: 1 }];
        TripleGenerator { max_perimeter, triples: BinaryHeap::from(roots) }
    }
}

impl Iterator for TripleGenerator {
    type Item = Triple;

    fn next(&mut self) -> Option<Self::Item> {
        let triple @ Triple { k, m, n } = self.triples.pop()?;

        let mut candidates = vec![Triple { k: k + 1, m, n }];

        if k == 1 {
            candidates.push(Triple { k: 1, m: 2 * m - n, n: m });
            candidates.push(Triple { k: 1, m: 2 * m + n, n: m });
            candidates.push(Triple { k: 1, m: 2 * n + m, n });
        }

        let candidates = candidates
            .into_iter()
            .filter(|t| t.perimeter() < self.max_perimeter)
            .collect::<Vec<_>>();

        self.triples.extend(candidates);

        Some(triple)
    }
}

fn main() {
    let tg = TripleGenerator::new(1_500_000 + 1);
    let mut perimeters = HashMap::new();

    for triple in tg {
        let value = perimeters.entry(triple.perimeter()).or_insert(0u64);
        *value += 1;
    }

    let solution = perimeters.into_iter().filter(|(_, v)| *v == 1).count();

    println!("{:?}", solution);
}
