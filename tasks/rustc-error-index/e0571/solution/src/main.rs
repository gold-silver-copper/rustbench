fn main() {
    let mut i = 1;
    fn satisfied(n: usize) -> bool { n % 23 == 0 }
    let result = loop {
        if satisfied(i) {
            break 2 * i;
        }
        i += 1;
    };
}
