use std::{
    collections::HashSet,
    iter::{once, successors},
    num::NonZeroU8,
};

use either::Either::{Left, Right};
use euler::nzu8;

#[derive(Clone)]
struct Gon {
    pub sides: usize,
    data: Vec<Option<NonZeroU8>>,
}

impl Gon {
    pub fn new(sides: usize) -> Self {
        Self { sides, data: vec![None; sides * 2] }
    }

    fn index_of(&self, side: usize, index: usize) -> usize {
        match index {
            0 => side + self.sides,
            1 => side,
            2 => (side + 1) % self.sides,
            _ => unreachable!(),
        }
    }

    pub fn at(&self, side: usize, index: usize) -> &Option<NonZeroU8> {
        &self.data[self.index_of(side, index)]
    }

    pub fn set_at(&mut self, side: usize, index: usize, value: Option<NonZeroU8>) {
        let index = self.index_of(side, index);
        self.data[index] = value;
    }
}

fn solve(
    gon: &Gon,
    used: &HashSet<u8>,
    total: Option<u8>,
    solutions: &mut HashSet<usize>,
    depth: usize,
) {
    if depth == gon.sides {
        let start_side = (0..gon.sides)
            .map(|s| u8::from(gon.at(s, 0).unwrap()))
            .enumerate()
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap()
            .0;

        let solution = (start_side..start_side + gon.sides)
            .map(|side| side % gon.sides)
            .flat_map(|side| (0..=2).map(move |i| gon.at(side, i).unwrap()))
            .map(u8::from)
            .fold(0usize, |ns, n| {
                let digits = successors(Some(n), |&n| (n >= 10).then(|| n / 10)).count();
                ns * 10usize.pow(digits as u32) + n as usize
            });

        solutions.insert(solution);

        return;
    }

    let len = gon.sides as u8 * 2;

    for xa in gon
        .at(depth, 0)
        .map(|a| Left(once(u8::from(a))))
        .unwrap_or_else(|| Right((1..=len).filter(|a| !used.contains(a))))
    {
        for xb in gon
            .at(depth, 1)
            .map(|b| Left(once(u8::from(b))))
            .unwrap_or_else(|| Right((1..=len).filter(|b| !used.contains(b))))
            .take_while(|&xb| total.map_or(true, |total| xa + xb < total))
            .filter(|&xb| xb != xa)
        {
            for xc in gon
                .at(depth, 2)
                .map(|c| Left(once(u8::from(c))))
                .unwrap_or_else(|| Right((1..=len).filter(|c| !used.contains(c))))
            {
                let sum = xa + xb + xc;

                if total.map_or(false, |total| sum > total) {
                    break;
                }

                if xc == xa || xc == xb || total.map_or(false, |total| sum < total) {
                    continue;
                }

                let used = used.iter().chain(&[xa, xb, xc]).copied().collect();
                let mut gon = gon.clone();

                for (index, xn) in [xa, xb, xc].into_iter().enumerate() {
                    gon.set_at(depth, index, Some(nzu8!(xn)));
                }

                solve(&gon, &used, Some(sum), solutions, depth + 1);
            }
        }
    }
}

fn main() {
    let mut solutions = HashSet::new();
    solve(&Gon::new(5), &HashSet::new(), None, &mut solutions, 0);
    let max = solutions.iter().filter(|&&v| v < 10_000_000_000_000_000).max();
    eprintln!("maximum = {:?}", max);
}
