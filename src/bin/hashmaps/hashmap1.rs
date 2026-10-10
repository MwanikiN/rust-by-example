use std::collections::HashMap;
use std::io;

fn main() {
    // Read inputs
    let mut product1 = String::new();
    io::stdin().read_line(&mut product1).expect("Failed to read line");
    let product1 = product1.trim().to_string();
    
    let mut price1_input = String::new();
    io::stdin().read_line(&mut price1_input).expect("Failed to read line");
    let price1: f64 = price1_input.trim().parse().expect("Failed to parse price");
    
    let mut product2 = String::new();
    io::stdin().read_line(&mut product2).expect("Failed to read line");
    let product2 = product2.trim().to_string();
    
    // TODO: Write your code below
    // Create a mutable HashMap, insert products, and print the required output
    let mut product_prices: HashMap<String, f64> = HashMap::new();
    product_prices.insert(product1.clone(), price1);
    product_prices.insert(product2.clone(), 0.0);

    println!("Inserted Laptop at ${}", product_prices[&product1]);
    println!("Inserted Mouse at ${}", product_prices[&product2]);
    
    product_prices.insert(product2.clone(), 15.99);

    println!("Inserted Mouse at ${}", product_prices[&product2]);
}