// TODO: Define a generic Stack<T> struct that uses a Vec<T> internally
pub struct Stack<T> {
    elements: Vec<T>
}

// TODO: Implement methods for Stack<T>
impl<T> Stack<T> {
    // TODO: Implement new() - creates an empty stack
    pub fn new() -> Self {
       Self{ elements: Vec::new(),}
    }
    // TODO: Implement push() - adds an element to the top of the stack
    pub fn push(&mut self, element: T) {
        let _ = &self.elements.push(element);
    }
    // TODO: Implement pop() - removes and returns the top element as Option<T>
    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }
}
