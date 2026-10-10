trait Foo<'foo>: 'foo {}
trait Bar<'bar>: 'bar {}

trait FooBar<'foo, 'bar>: Foo<'foo> + Bar<'bar> {}

struct Baz<'foo, 'bar> {
    baz: dyn FooBar<'foo, 'bar>,
}
