trait Tr<'a, 'b> {}

fn foo<T>(t: T)
where
    for<'a, 'b> &'a T: Tr<'a, 'b>,
{
}
