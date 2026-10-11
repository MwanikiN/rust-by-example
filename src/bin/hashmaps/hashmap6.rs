use std::collections::HashMap;
use std::io;

fn main() {
    // Read input
    let mut item_to_sell = String::new();
    io::stdin().read_line(&mut item_to_sell).expect("Failed to read line");
    let item_to_sell = item_to_sell.trim();

    let mut quantity_to_sell_input = String::new();
    io::stdin().read_line(&mut quantity_to_sell_input).expect("Failed to read line");
    let quantity_to_sell: i32 = quantity_to_sell_input.trim().parse().expect("Invalid number");

    let mut item_names_input = String::new();
    io::stdin().read_line(&mut item_names_input).expect("Failed to read line");
    let item_names_input = item_names_input.trim();

    let mut prices_input = String::new();
    io::stdin().read_line(&mut prices_input).expect("Failed to read line");
    let prices_input = prices_input.trim();

    let mut quantities_input = String::new();
    io::stdin().read_line(&mut quantities_input).expect("Failed to read line");
    let quantities_input = quantities_input.trim();

    // Create a HashMap to store inventory
    let mut inventory_list: HashMap<String, (f64, i32)> = HashMap::new();

    let names: Vec<String> = item_names_input
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    let prices: Vec<f64> = prices_input
        .split(',')
        .map(|s| s.trim().parse::<f64>().unwrap())
        .collect();

    let quantities: Vec<i32> = quantities_input
        .split(',')
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect();

    for i in 0..names.len() {
        inventory_list.insert(names[i].clone(), (prices[i], quantities[i]));
    }

    // Check if the item exists and handle the sale
    match inventory_list.get(item_to_sell) {
        Some(&(price, current_quantity)) => {
            if current_quantity >= quantity_to_sell {
                let new_quantity = current_quantity - quantity_to_sell;
                let total_price = price * quantity_to_sell as f64;

                inventory_list.insert(
                    item_to_sell.to_string(),
                    (price, new_quantity),
                );

                println!("Sold {} {}", quantity_to_sell, item_to_sell);
                println!("Total: ${:.2}", total_price);
                println!("Remaining stock: {}", new_quantity);
            } else {
                println!("Insufficient stock for {}", item_to_sell);
                println!("Available: {}", current_quantity);
            }
        }
        None => println!("{} is not in stock", item_to_sell),
    }
}