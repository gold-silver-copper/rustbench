struct Foo;
struct Bar;

impl Foo for Bar {}
fn baz<T: Foo>(t: T) {}
