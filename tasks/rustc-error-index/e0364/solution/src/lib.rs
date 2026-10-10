mod a {
    pub fn foo() {}

    mod a {
        pub use super::foo;
    }
}
