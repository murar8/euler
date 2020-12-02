use std::{
    cmp::{max, min},
    collections::HashSet,
};

type Fraction = (usize, usize);

fn ones(n: usize) -> usize { n % 10 }

fn tens(n: usize) -> usize { (n / 10) % 10 }

fn is_equivalent((na, da): Fraction, (nb, db): Fraction) -> bool {
    if da == 0 || db == 0 {
        false
    } else {
        na as f64 / da as f64 == nb as f64 / db as f64
    }
}

fn is_cancelling((n, d): Fraction) -> bool {
    if tens(n) == tens(d) {
        is_equivalent((n, d), (ones(n), ones(d)))
    } else if tens(n) == ones(d) {
        is_equivalent((n, d), (ones(n), tens(d)))
    } else if ones(n) == tens(d) {
        is_equivalent((n, d), (tens(n), ones(d)))
    } else {
        false
    }
}

fn is_trivial((n, d): Fraction) -> bool { ones(n) == ones(d) }

fn product(fs: &[Fraction]) -> Fraction {
    fs.iter().fold((1, 1), |(np, dp), (nx, dx)| (np * nx, dp * dx))
}

fn simplify((mut n, mut d): Fraction) -> Fraction {
    loop {
        let max = max((n as f64).sqrt() as usize, (d as f64).sqrt() as usize);

        let mut done = true;

        for p in 2..=max {
            if n % p == 0 && n % p == 0 {
                n /= p;
                d /= p;
                done = false;
                break;
            }
        }

        if done {
            return (n, d);
        }
    }
}

fn main() {
    let mut fractions = Vec::new();

    for denom in 11..=99 {
        for nom in 10..denom {
            if !is_trivial((nom, denom)) && is_cancelling((nom, denom)) {
                fractions.push((nom, denom))
            }
        }
    }

    let product = product(&fractions);

    println!("{:?}", simplify(product))
}
