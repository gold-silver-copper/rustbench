struct Bar;

pub trait PubTr {
    type Alias;
}

impl PubTr for u8 {
    type Alias = Bar;
}

fn main() {}
