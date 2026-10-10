fn bar(x: &mut i32) {}
fn foo(a: &mut i32) {
    bar(a);
    let y = &a;
    println!("{}", y);
}
