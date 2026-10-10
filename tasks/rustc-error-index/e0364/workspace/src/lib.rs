mod a {
    fn foo() {}

    mod a {
        pub use super::foo;
    }
}
