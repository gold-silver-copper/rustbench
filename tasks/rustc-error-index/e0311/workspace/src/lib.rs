fn no_restriction<T>(x: &()) -> &() {
    with_restriction::<T>(x)
}

fn with_restriction<'a, T: 'a>(x: &'a ()) -> &'a () {
    x
}
