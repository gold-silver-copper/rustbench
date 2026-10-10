pub mod label;
pub mod shapes;

pub trait Draw {
    fn draw(&self) -> String;
}
