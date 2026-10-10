#[macro_use(debug_assert)]
extern crate core;

mod foo {
    fn run_macro() { debug_assert!(true); }
}
fn main() {}
