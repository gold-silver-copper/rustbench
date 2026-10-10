fn is_123(x: [u32; 3]) -> bool {
    match x {
        [1, 2, ..] => true,
        _ => false
    }
}
