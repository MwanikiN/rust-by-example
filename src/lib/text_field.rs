use super::draw::Draw;

// TODO: Define a TextField struct with a public `content` field (String)
// TODO: Implement the Draw trait for TextField
//       draw() should print: Text: {content}
pub struct TextField {
    pub content: String
}

impl Draw for TextField {
    fn draw(&self) -> String {
        format!("Text: {}",self.content)
    }
}