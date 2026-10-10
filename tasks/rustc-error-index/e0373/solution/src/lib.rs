fn foo() -> Box<dyn Fn(u32) -> u32> {
    let x = 0u32;
    Box::new(move |y| x + y)
}
