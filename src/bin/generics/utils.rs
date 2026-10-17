// TODO: Implement wrap_in_pair<T> function
// - Takes a single value of type T
// - Returns a tuple (T, T) containing the value twice
// - Hint: The type T needs to implement Clone trait
// - Use the syntax: pub fn wrap_in_pair<T: Clone>(value: T) -> ...
pub fn wrap_in_pair<T: Clone>(val: T) -> (T, T) {
    (val.clone(), val)
}
// TODO: Implement swap<T, U> function
// - Takes two values of potentially different types T and U
// - Returns them in reversed order as a tuple (U, T)
// - Hint: Use two generic type parameters
pub fn swap<T, U>(val1: T, val2: U) -> (U, T) {
    (val2, val1)
}