use foo::baz;
use bar::baz;

fn main() {}

mod foo {
    pub struct baz;
}

mod bar {
    pub mod baz {}
}
