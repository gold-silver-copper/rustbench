fn no_restriction<'a, T: 'a>(x: &'a ()) -> &'a () {
    with_restriction::<T>(x)
}

fn with_restriction<'a, T: 'a>(x: &'a ()) -> &'a () {
    x
}
