use super::draw::Draw;

// TODO: Define a Button struct with public `label` and `width` fields
// TODO: Implement the Draw trait for Button
//       draw() should print: Button: {label} ({width}px)
pub struct Button {
    pub label: String,
    pub width: u32
}

impl Draw for Button {
    fn draw(&self) -> String {
        format!("Button: {} ({}px)", self.label, self.width)
    }
}