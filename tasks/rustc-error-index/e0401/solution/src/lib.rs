fn foo<T>(x: T) {
    fn bar<T>(y: T) {
    }
    bar(x);
}
