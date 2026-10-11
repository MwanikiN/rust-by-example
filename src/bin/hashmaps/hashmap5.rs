use std::collections::HashMap;
use std::io;

fn main() {
    // Read item name
    let mut item_name = String::new();
    io::stdin().read_line(&mut item_name).expect("Failed to read line");
    let item_name = item_name.trim().to_string();
    
    // Read price
    let mut price_input = String::new();
    io::stdin().read_line(&mut price_input).expect("Failed to read line");
    let price: f64 = price_input.trim().parse().expect("Failed to parse price");
    
    // Read quantity
    let mut quantity_input = String::new();
    io::stdin().read_line(&mut quantity_input).expect("Failed to read line");
    let quantity: i32 = quantity_input.trim().parse().expect("Failed to parse quantity");
    
    // TODO: Write your code below
    // Create a HashMap to store inventory
    let mut inventory_list: HashMap<String, (f64, i32)> = HashMap::new();
    // Add the item to the inventory
    inventory_list.insert(item_name, (price, quantity));
    // Print the confirmation messages
    for (commodity, details) in inventory_list {
        println!("Added {} to inventory", commodity);
        println!("Price: ${}", details.0);
        println!("Quantity: {}", details.1);
    }
    
}