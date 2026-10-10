fn foo() {
    static mut X: u32 = 4;
    const Y: u32 = 5;
    fn bar() -> u32 {
        unsafe {
            X = 3;
        }
        Y
    }
}
