use num_bigint::ToBigUint;
use std::collections::HashSet;

fn main() {
    let mut terms = HashSet::new();

    for a in 2u32..=100 {
        for b in 2..=100 {
            terms.insert(a.to_biguint().unwrap().pow(b));
        }
    }

    println!("{:?}", terms.len());
}
