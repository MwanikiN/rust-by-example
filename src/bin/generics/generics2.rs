mod utils;

use utils::{swap, wrap_in_pair};
use std::io;

fn main() {
    // Read the four inputs
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let num1: i32 = input1.trim().parse().expect("Invalid integer");

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let str1 = input2.trim().to_string();

    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read line");
    let num2: i32 = input3.trim().parse().expect("Invalid integer");

    let mut input4 = String::new();
    io::stdin().read_line(&mut input4).expect("Failed to read line");
    let str2 = input4.trim().to_string();

    // TODO: Use wrap_in_pair with num1 and print the result
    // Format: Pair of ints: ({value}, {value})
    let pair_of_ints = wrap_in_pair(num1);
    println!("Pair of ints: ({}, {})", pair_of_ints.0, pair_of_ints.1);
    // TODO: Use wrap_in_pair with str1 and print the result
    // Format: Pair of strings: ({value}, {value})
    let pair_of_strings = wrap_in_pair(str1);
    println!("Pair of strings: ({}, {})", pair_of_strings.0, pair_of_strings.1);
    // TODO: Use swap with num2 and str2, then print the swapped result
    // Format: Swapped: ({string}, {int})
    let swapped = swap(num2, str2);
    println!("Swapped: ({}, {})", swapped.0, swapped.1);
}
