trait Trait { type X; }

struct Struct;
impl Trait for Struct {
    type X = u32;
}

fn main() {
    let foo: <Struct as Trait>::X;
}
