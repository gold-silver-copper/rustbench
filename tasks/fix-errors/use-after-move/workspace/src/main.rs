fn main() {
    let (_, line) = task::place_order("Ana".to_string(), vec!["tea".into(), "scones".into()]);
    println!("{line}");
}
