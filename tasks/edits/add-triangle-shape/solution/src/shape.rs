use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shape {
    Circle { r: f64 },
    Rect { w: f64, h: f64 },
    /// A triangle given by its three side lengths.
    Triangle { a: f64, b: f64, c: f64 },
}

impl Shape {
    pub fn area(&self) -> f64 {
        match *self {
            Shape::Circle { r } => PI * r * r,
            Shape::Rect { w, h } => w * h,
            Shape::Triangle { a, b, c } => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }

    pub fn perimeter(&self) -> f64 {
        match *self {
            Shape::Circle { r } => 2.0 * PI * r,
            Shape::Rect { w, h } => 2.0 * (w + h),
            Shape::Triangle { a, b, c } => a + b + c,
        }
    }

    /// The same shape scaled by `k` in every direction.
    pub fn scaled(&self, k: f64) -> Shape {
        match *self {
            Shape::Circle { r } => Shape::Circle { r: r * k },
            Shape::Rect { w, h } => Shape::Rect { w: w * k, h: h * k },
            Shape::Triangle { a, b, c } => Shape::Triangle {
                a: a * k,
                b: b * k,
                c: c * k,
            },
        }
    }
}
