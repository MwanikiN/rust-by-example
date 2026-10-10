use std::io;

// TODO: Create your divide function here that returns Result<f64, &'static str>
fn divide (n: f64, m: f64) -> Result<f64, &'static str> {
    if m == 0.0 {
        Err("Division by zero")
    }
    else {
        Ok(n / m)
    }
}

fn main() {
    // Read first input (dividend)
    let mut dividend_input = String::new();
    io::stdin().read_line(&mut dividend_input).expect("Failed to read line");
    let dividend: f64 = dividend_input.trim().parse().expect("Invalid number");
    
    // Read second input (divisor)
    let mut divisor_input = String::new();
    io::stdin().read_line(&mut divisor_input).expect("Failed to read line");
    let divisor: f64 = divisor_input.trim().parse().expect("Invalid number");
    
    // TODO: Call your divide function and use match to handle the Result
    match divide(dividend, divisor) {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e)
    }
    // Print "Result: [value]" for Ok case
    
    // Print "Error: [error]" for Err case
}