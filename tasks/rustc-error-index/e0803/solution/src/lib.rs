trait DataAccess<'a, T> {
    fn get_ref(&'a self) -> T;
}

struct Container<'a> {
    value: &'a f64,
}

impl<'a> DataAccess<'a, &'a f64> for Container<'a> {
    fn get_ref(&'a self) -> &'a f64 {
        self.value
    }
}
