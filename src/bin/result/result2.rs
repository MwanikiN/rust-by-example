use std::io;

fn main() {
    // Read the first input (number)
    let mut number_input = String::new();
    io::stdin().read_line(&mut number_input).expect("Failed to read line");
    let number_input = number_input.trim();
    
    // Parse the number to i32
    let parsed_number: i32 = number_input.parse().expect("Failed to parse number");
    
    // Read the second input (status)
    let mut status = String::new();
    io::stdin().read_line(&mut status).expect("Failed to read line");
    let status = status.trim();
    
    // TODO: Create a Result<i32, &'static str> based on the status
    // If status is "success", create Ok(parsed_number)
    // If status is "failure", create Err("Operation failed")
    let result_type: Result<i32, &'static str> = if status == "success" {
        Ok(parsed_number)
    }
    else {
        Err("Operation failed")
    };    
    // TODO: Use .is_ok() and .is_err() to check the result and print the appropriate message
    if result_type.is_ok() {
        println!("Success detected");
    }
    if result_type.is_err() {
        println!("Error detected");
    }
}