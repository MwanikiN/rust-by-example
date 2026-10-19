// TODO: Define a public generic Point<T> struct with public fields x and y
pub struct Point<T> {
   pub x: T,
   pub y: T
}

// TODO: Implement methods for Point<T>
impl<T> Point<T> {
    // TODO: Implement the new associated function
    // pub fn new(...) -> Self { ... }
    pub fn new(x: T, y: T) -> Self {
        Point{
            x,
            y
        }
    }
}

// TODO: Implement the translate method
// This requires T to support addition and copying
// Use the bound: T: std::ops::Add<Output = T> + Copy
impl<T: std::ops::Add<Output = T> + Copy> Point<T> {
    // TODO: Implement translate method that returns a new Point
    // pub fn translate(...) -> Self { ... }
    pub fn translate(&mut self, dx: T, dy: T) -> Self {
        Point {
            x: self.x + dx,
            y: self.y + dy
        }
    }
}
