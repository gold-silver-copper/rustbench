trait Bar {
    fn f(self);
}

impl Bar for i32 {
    fn f(self) {}
}

fn main() {
    let b: Box<dyn Bar> = Box::new(0i32);
    b.f();
}
