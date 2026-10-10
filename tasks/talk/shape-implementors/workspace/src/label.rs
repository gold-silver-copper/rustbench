use crate::Draw;

/// Text drawn next to a shape. Not a shape itself: it has no area.
pub struct Label {
    pub text: String,
}

impl Draw for Label {
    fn draw(&self) -> String {
        format!("[{}]", self.text)
    }
}
