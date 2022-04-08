use std::collections::HashMap;

use euler::permutations::Permutations;

fn compose_number(digits: &[u64]) -> u64 {
    digits.iter().enumerate().map(|(i, n)| n * 10u64.pow(i as u32)).sum()
}

fn find_pandigital_product(digits: &[u64], store: &mut HashMap<u64, (u64, u64)>) {
    for a_end in 1..=7 {
        for b_end in (a_end + 1)..8 {
            let a = compose_number(&digits[0..a_end]);
            let b = compose_number(&digits[a_end..b_end]);
            let r = compose_number(&digits[b_end..]);

            if a * b == r {
                store.insert(r, (a, b));
            }
        }
    }
}

fn main() {
    let digits = (1..=9).collect::<Vec<_>>();
    let perms = Permutations::new(digits, 9);
    let mut products = HashMap::new();

    for p in perms {
        find_pandigital_product(p.as_slice(), &mut products)
    }

    println!("{:?}", products.keys().copied().sum::<u64>())
    //
}
