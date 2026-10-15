
mod locker;

use locker::Locker;
use std::io;

fn main() {
    let mut input = String::new();

    // Read initial code
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let initial_code: u32 = input.trim().parse().expect("Invalid code");

    // Read contents
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let contents = input.trim().to_string();

    // Read first attempted code
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let first_attempt: u32 = input.trim().parse().expect("Invalid code");

    // Read new code
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let new_code: u32 = input.trim().parse().expect("Invalid code");

    // Read second attempted code
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let second_attempt: u32 = input.trim().parse().expect("Invalid code");

    // Create the locker
    let mut locker = Locker::new(initial_code, contents);

    // First access attempt
    println!(
        "Attempt with {:04}: {}",
        first_attempt,
        locker.get_contents(first_attempt)
    );

    // Change the code
    locker.set_code(new_code);
    println!("Code changed");

    // Second access attempt
    println!(
        "Attempt with {:04}: {}",
        second_attempt,
        locker.get_contents(second_attempt)
    );
}
