fn main() {
    let w = || {
        for _ in 0..10 {
            break;
        }
    };

    w();
}
