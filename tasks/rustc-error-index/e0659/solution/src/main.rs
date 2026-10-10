pub mod moon {
    pub fn foo() {}
}

pub mod earth {
    pub fn foo() {}
}

mod collider {
    pub use crate::moon;
    pub use crate::earth;
}

fn main() {
    crate::collider::moon::foo();
    crate::collider::earth::foo();
}
