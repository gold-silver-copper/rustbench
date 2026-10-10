fn main() {
    fn foo() -> i32 { 22 }
    fn bar(x: &i32) -> &i32 { x }
    let p = bar(&foo());
    let q = *p;
}
