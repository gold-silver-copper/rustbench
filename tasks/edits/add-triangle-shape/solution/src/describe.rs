use crate::shape::Shape;

pub fn name(shape: &Shape) -> &'static str {
    match shape {
        Shape::Circle { .. } => "circle",
        Shape::Rect { .. } => "rectangle",
        Shape::Triangle { .. } => "triangle",
    }
}

/// e.g. "rectangle: area 6.00, perimeter 10.00"
pub fn describe(shape: &Shape) -> String {
    format!(
        "{}: area {:.2}, perimeter {:.2}",
        name(shape),
        shape.area(),
        shape.perimeter()
    )
}

/// Total area of all shapes.
pub fn total_area(shapes: &[Shape]) -> f64 {
    shapes.iter().map(Shape::area).sum()
}
