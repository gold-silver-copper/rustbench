fn main() {
    for code in ["SU", "RA", "SN", "XX"] {
        println!("{code}: {}", task::forecast(code));
    }
}
