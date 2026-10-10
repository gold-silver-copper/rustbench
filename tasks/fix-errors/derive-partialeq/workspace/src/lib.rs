#[derive(Debug, Clone, Copy)]
pub enum Light {
    Red,
    Yellow,
    Green,
}

pub fn next(l: Light) -> Light {
    match l {
        Light::Red => Light::Green,
        Light::Green => Light::Yellow,
        Light::Yellow => Light::Red,
    }
}

/// True when the light lets cars go.
pub fn can_go(l: Light) -> bool {
    l == Light::Green
}
