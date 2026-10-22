
// mod draw;
// mod text_field;
// mod button;
// mod screen;

use std::io;
use rust_by_example::lib::text_field::TextField;
use rust_by_example::lib::button::Button;
use rust_by_example::lib::screen::Screen;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read text field content");
    let text = input.trim().to_string();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read button label");
    let label = input.trim().to_string();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read button width");
    let width: u32 = input.trim().parse().expect("Invalid button width");

    let mut screen = Screen::new();

    screen.add(Box::new(TextField { content: text }));

    screen.add(Box::new(Button { label, width }));

    screen.run();
}
