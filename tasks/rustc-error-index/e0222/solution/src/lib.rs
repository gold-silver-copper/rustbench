pub trait Vehicle {
    type Color;
}

pub trait Box {
    type Color;
}

pub trait BoxCar : Box + Vehicle {}

fn foo<CAR, COLOR>(
    c: CAR,
) where
    CAR: BoxCar,
    CAR: Vehicle<Color = COLOR>,
    CAR: Box<Color = COLOR>
{}
