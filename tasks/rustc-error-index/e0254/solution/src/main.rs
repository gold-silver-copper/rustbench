extern crate core as libcore;

mod foo {
    pub trait core {
        fn do_something();
    }
}

use foo::core;

fn main() {}
