fn underscore_lifetime<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

fn without_explicit_lifetime<'a, T>()
where
    T: Iterator<Item = &'a u32>,
{
}

fn without_hrtb<T>()
where
    T: for<'foo> Into<&'foo u32>,
{
}
