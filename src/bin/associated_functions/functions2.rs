use std::io;

// TODO: Define the TextStyle struct here
struct TextStyle {
    bold: bool,
    italic: bool,
    size: u32
}

// TODO: Add the implementation block for TextStyle with:
// - new() associated function
impl TextStyle {
    fn new() -> TextStyle {
    TextStyle {
        bold: false,
        italic: false,
        size: 12
    }
}   
    // - set_bold() method
    fn  set_bold(&mut self, val: bool) -> &mut Self {
        self.bold = val;
        self
    }
    // - set_italic() method
    fn set_italic(&mut self, val: bool) ->&mut Self {
        self.italic = val;
        self
    }
    // - set_size() method
    fn set_size(&mut self, value: u32) -> &mut Self {
        self.size = value;
        self
    }
}





fn main() {
    let mut input = String::new();
    
    // Read bold setting
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let bold: bool = input.trim().parse().expect("Invalid bool");
    input.clear();
    
    // Read italic setting
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let italic: bool = input.trim().parse().expect("Invalid bool");
    input.clear();
    
    // Read size setting
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let size: u32 = input.trim().parse().expect("Invalid u32");
    
    // TODO: Create a mutable TextStyle using new(), then use method chaining
    // to apply all three settings in a single expression
    let mut text = TextStyle::new();
    text.set_bold(bold)
        .set_italic(italic)
        .set_size(size);
    
    // TODO: Print the result in the format: Bold: {bold}, Italic: {italic}, Size: {size}
    println!("Bold: {}, Italic: {}, Size: {}", text.bold, text.italic, text.size);
}