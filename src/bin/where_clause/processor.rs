// TODO: Define a public trait Summarize with a method:
// fn summary(&self) -> String
pub trait Summarize {
    fn summary(&self) -> String;
}
// TODO: Define a public trait Validate with a method:
// fn validate(&self) -> bool
pub trait Validate {
    fn validate(&self) -> bool;
}
// TODO: Create a public generic function process_items<T, U>
// Use a where clause to specify:
// - T must implement Clone and Summarize
// - U must implement Validate
// The function should:
// 1. Print the summary of the first item
// 2. Print "Valid: {result}" where result is the validation of the second item
pub fn process_items<T, U>(item1: T, item2: U) 
where 
    T: Clone + Summarize,
    U: Validate,
{
    println!("Article: {}",item1.summary());
    println!("Valid: {}",item2.validate());
}