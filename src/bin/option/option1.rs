use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the comma-separated numbers
    let numbers_input = lines.next().unwrap().unwrap().trim().to_string();
    
    // Read the target number
    let target_input = lines.next().unwrap().unwrap().trim().to_string();
    
    // Parse the comma-separated numbers into a vector
    let numbers: Vec<i32> = numbers_input
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();
    
    // Parse the target number
    let target: i32 = target_input.parse().unwrap();
    
    // TODO: Write your code below
    // Use .iter().position() to find the target and match to handle the result
    let pos = numbers.iter().position(|&x| x==target);

    match pos {
        Some(i) => println!("Found at index {}", i),
        None => println!("Not found")
    }
    
}