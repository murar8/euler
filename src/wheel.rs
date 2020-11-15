pub struct Wheel {
    pub size: u64,
    pub basis: Vec<u64>,

    basis_index: usize,

    distances: Vec<u64>,
    index: usize,
    value: u64,
}

impl Wheel {
    pub fn new(basis: Vec<u64>) -> Self {
        let size = basis.iter().product::<u64>();
        let is_coprime = |n: &u64| basis.iter().all(|b| n % b != 0);
        let spokes = (1..=size + 1).filter(is_coprime).collect::<Vec<_>>();
        let distances = spokes.windows(2).map(|ss| ss[1] - ss[0]).collect();
        Wheel { basis, size, basis_index: 0, distances, index: 0, value: 1 }
    }
}

impl Iterator for Wheel {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.basis_index < self.basis.len() {
            self.basis_index += 1;
            Some(self.basis[self.basis_index - 1])
        } else {
            self.value += self.distances[self.index];
            self.index = (self.index + 1) % self.distances.len();
            Some(self.value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_wheel() {
        let wheel = Wheel::new(vec![2, 3, 5]);

        assert_eq!(
            wheel.take_while(|n| *n < 100).collect::<Vec<_>>(),
            [
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 49, 53, 59, 61, 67, 71, 73,
                77, 79, 83, 89, 91, 97
            ]
        )
    }

    #[bench]
    fn bench_wheel_1_000_000(b: &mut Bencher) {
        b.iter(move || {
            let mut wheel = Wheel::new(vec![2, 3, 5]);
            let _ = wheel.by_ref().take_while(|n| *n < 1_000_000).for_each(drop);
        });
    }

    #[bench]
    fn bench_wheel_10_000_000(b: &mut Bencher) {
        b.iter(move || {
            let mut wheel = Wheel::new(vec![2, 3, 5]);
            let _ = wheel.by_ref().take_while(|n| *n < 10_000_000).for_each(drop);
        });
    }
}
