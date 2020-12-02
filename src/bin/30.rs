use std::collections::HashSet;

fn solve_rec(power: u32, result: &mut HashSet<u64>, digits: &[u64]) {
    let end = if digits.is_empty() { 1 } else { 0 };

    for next in (end..10).rev() {
        let digits = digits.iter().chain(&[next]).copied().collect::<Vec<_>>();
        let sum_of_powers = digits.iter().map(|d| d.pow(power)).sum::<u64>();
        let value = digits.iter().fold(0u64, |acc, d| acc * 10 + d);

        if sum_of_powers == value && digits.len() > 1 {
            result.insert(value);
        }

        if sum_of_powers < value {
            break;
        }

        solve_rec(power, result, digits.as_slice());
    }
}

fn solve(power: u32) -> HashSet<u64> {
    let mut result = HashSet::new();
    solve_rec(power, &mut result, &[]);
    result
}

fn main() {
    println!("{:?}", solve(5).iter().sum::<u64>());
}
