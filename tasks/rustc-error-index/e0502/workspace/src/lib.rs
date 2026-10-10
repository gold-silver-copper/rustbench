fn bar(x: &mut i32) {}
fn foo(a: &mut i32) {
    let y = &a;
    bar(a);
    println!("{}", y);
}
