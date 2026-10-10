fn is_123<const N: usize>(x: [u32; N]) -> bool {
    match x {
        [1, 2, ..] => true,
        _ => false
    }
}
