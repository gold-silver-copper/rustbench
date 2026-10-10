#[cfg_attr(linux, repr(u32))]
#[cfg_attr(not(linux), repr(u64))]
enum Repr { A }
