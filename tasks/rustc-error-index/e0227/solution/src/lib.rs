trait Foo<'foo>: 'foo {}
trait Bar<'bar>: 'bar {}

trait FooBar<'foo, 'bar>: Foo<'foo> + Bar<'bar> {}

struct Baz<'foo, 'bar, 'baz>
where
    'baz: 'foo + 'bar,
{
    obj: dyn FooBar<'foo, 'bar> + 'baz,
}
