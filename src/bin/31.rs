use std::cmp::Ordering;

const COINS: [u32; 8] = [200, 100, 50, 20, 10, 5, 2, 1];

fn solve(total: u32, coins: &[u32]) -> u32 {
    let mut result = 0;

    for (i, coin) in coins.iter().enumerate() {
        for amount in 1.. {
            match (coin * amount).cmp(&total) {
                | Ordering::Greater => {
                    break;
                }
                | Ordering::Equal => {
                    result += 1;
                }
                | Ordering::Less => {
                    result += solve(total - (coin * amount), &coins[i + 1..]);
                }
            }
        }
    }

    result
}

fn main() {
    println!("{:?}", solve(200, &COINS));
}
