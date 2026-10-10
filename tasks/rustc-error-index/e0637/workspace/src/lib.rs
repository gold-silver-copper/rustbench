fn underscore_lifetime<'_>(str1: &'_ str, str2: &'_ str) -> &'_ str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

fn without_explicit_lifetime<T>()
where
    T: Iterator<Item = &u32>,
{
}

fn without_hrtb<T>()
where
    T: Into<&u32>,
{
}
