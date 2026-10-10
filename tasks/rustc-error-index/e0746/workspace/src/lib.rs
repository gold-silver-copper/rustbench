trait T {
    fn bar(&self);
}
struct S(usize);
impl T for S {
    fn bar(&self) {}
}

fn foo() -> dyn T {
    S(42)
}
