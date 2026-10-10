fn main() {
    mod foo {
        #[macro_use(debug_assert)]
        extern crate core;
        fn run_macro() { debug_assert!(true); }
    }
}
