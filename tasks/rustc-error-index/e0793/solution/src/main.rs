fn main() {
    #[repr(packed)]
    pub struct Foo {
        field1: u64,
        field2: u8,
    }

    unsafe {
        let foo = Foo { field1: 0, field2: 0 };

        let ptr = std::ptr::addr_of!(foo.field1);
        let val = unsafe { ptr.read_unaligned() };

        let copy = foo.field1;
        println!("{}", copy);

        println!("{}", { foo.field1 });

        println!("{}", foo.field2);
    }
}
