fn solve(max_size: u64) -> u64 {
    let mut count = 1u64;
    let mut sum = 1u64;

    for size in (3..).step_by(2) {
        if size > max_size {
            break;
        }

        for _ in 1..=4 {
            count += size - 1;
            sum += count;
        }
    }

    sum
}

fn main() {
    println!("{:?}", solve(1001));
}
