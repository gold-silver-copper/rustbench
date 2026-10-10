fn main() {
    #[repr(packed)]
    pub struct Foo {
        field1: u64,
        field2: u8,
    }

    unsafe {
        let foo = Foo { field1: 0, field2: 0 };
        let val = foo.field1;
        let val = &foo.field1;
        println!("{}", foo.field1);
    }
}
