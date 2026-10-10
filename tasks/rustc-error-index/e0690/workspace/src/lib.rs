#[repr(transparent)]
struct LengthWithUnit<U> {
    value: f32,
    unit: U,
}
