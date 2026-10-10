fn takes_u8(_: u8) {}

fn main() {
    unsafe { takes_u8(::std::mem::transmute(0i8)); }
    unsafe { takes_u8(0u8); }
}
