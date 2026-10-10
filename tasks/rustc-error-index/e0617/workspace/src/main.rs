fn main() {
    use std::os::raw::{c_char, c_int};
    extern "C" {
        fn printf(format: *const c_char, ...) -> c_int;
    }

    unsafe {
        printf("%f\n\0".as_ptr() as _, 0f32);
    }
}
