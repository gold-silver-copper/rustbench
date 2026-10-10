fn main() {
    let x = 0u8;
    x as u32;

    let v = core::ptr::null::<u8>();
    v as *const i8;
}
