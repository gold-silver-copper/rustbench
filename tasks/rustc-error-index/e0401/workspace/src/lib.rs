fn foo<T>(x: T) {
    fn bar(y: T) {
    }
    bar(x);
}
