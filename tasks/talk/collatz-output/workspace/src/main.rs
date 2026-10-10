/// Steps for `n` to reach 1 under the Collatz rule.
fn steps(mut n: u64) -> u32 {
    let mut count = 0;
    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        count += 1;
    }
    count
}

fn main() {
    println!("{}", steps(27));
}
