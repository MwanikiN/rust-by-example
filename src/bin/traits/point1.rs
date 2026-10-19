use std::fmt;

// TODO: Define a public Point struct with public x and y fields (both i32)
// Derive Debug and PartialEq traits automatically
#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32
}
// TODO: Implement the Display trait for Point manually
// The format should be: (x, y)
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Write the formatted output
        write!(f, "({}, {})",self.x, self.y)
    }
}
