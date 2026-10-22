// mod draw;
// mod text_field;
// mod button;

use rust_by_example::lib::draw::Draw;
use rust_by_example::lib::text_field::TextField;
use rust_by_example::lib::button::Button;

// TODO: Define a public struct called Label with a public text field (String)
pub struct Label {
    pub text: String
}
// TODO: Implement the Draw trait for Label
// The draw method should print the label's text
impl Draw for Label {
    fn draw(&self) -> String {
        self.text.clone()
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let text = input.trim().to_string();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let label: String = input.trim().to_string();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let width: u32 = input.trim().parse().unwrap();

    // TODO: Create a Label instance with the input text
    // TODO: Call the draw() method on the label
    // let draw_text: Label = Label {text};
    // println!("{}",draw_text.draw());
    let text_field = TextField {content: text};
    println!("{}",text_field.draw());

    let button: Button = Button {label, width};
    println!("{}",button.draw());
}
