trait Tr<'a, 'b> {}

fn foo<T>(t: T)
where
    for<'a> &'a T: for<'b> Tr<'a, 'b>,
{
}
