use std::io;

fn main() {
    // Read product name
    let mut product_name = String::new();
    io::stdin().read_line(&mut product_name).expect("Failed to read line");
    let product_name = product_name.trim();
    
    // Read stock status
    let mut stock_status = String::new();
    io::stdin().read_line(&mut stock_status).expect("Failed to read line");
    let stock_status = stock_status.trim();
    
    // TODO: Write your code below
    // Create an Option<String> based on stock_status
    let product_availability: Option<String> = if stock_status == "yes" {
        Some(product_name.to_string())
    }
    else {
        None
    };
    // Use .expect() to extract the product name
    if product_availability.is_some() {
        println!("Product available: {}", product_availability.expect("Product should be in stock"));
    }
    // Print the result in the format: Product available: [product_name]
    
}