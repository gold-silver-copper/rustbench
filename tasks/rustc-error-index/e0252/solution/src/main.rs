use foo::baz as foo_baz;
use bar::baz;

fn main() {}

mod foo {
    pub struct baz;
}

mod bar {
    pub mod baz {}
}
