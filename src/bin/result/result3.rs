use std::io;

// TODO: Create the safe_divide function here
fn safe_divide(dividend: f64, divisor: f64) -> Result<f64, &'static str> {
    if divisor == 0.0 {
        Err("Cannot divide by zero")
    }
    else {
        Ok(dividend / divisor)
    }
}

fn main() {
    // Read the first input (dividend)
    let mut dividend_input = String::new();
    io::stdin().read_line(&mut dividend_input).expect("Failed to read line");
    let dividend: f64 = dividend_input.trim().parse().expect("Invalid number");
    
    // Read the second input (divisor)
    let mut divisor_input = String::new();
    io::stdin().read_line(&mut divisor_input).expect("Failed to read line");
    let divisor: f64 = divisor_input.trim().parse().expect("Invalid number");
    
    // TODO: Call safe_divide and use match to handle the Result
    // Print the appropriate output based on the result

    match safe_divide(dividend, divisor) {
        Ok(val) => println!("Division result: {}", val),
        Err(e) => println!("Error: {}", e)
    }
}