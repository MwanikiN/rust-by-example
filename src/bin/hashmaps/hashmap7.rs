
use std::collections::HashMap;
use std::io;

fn main() {
    // Read inputs
    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("Failed to read line");
    let command = command.trim();

    let mut item_name = String::new();
    io::stdin().read_line(&mut item_name).expect("Failed to read line");
    let item_name = item_name.trim().to_string();

    let mut value_input = String::new();
    io::stdin().read_line(&mut value_input).expect("Failed to read line");
    let value_input = value_input.trim();

    let mut items_input = String::new();
    io::stdin().read_line(&mut items_input).expect("Failed to read line");
    let items_input = items_input.trim();

    let mut prices_input = String::new();
    io::stdin().read_line(&mut prices_input).expect("Failed to read line");
    let prices_input = prices_input.trim();

    let mut quantities_input = String::new();
    io::stdin().read_line(&mut quantities_input).expect("Failed to read line");
    let quantities_input = quantities_input.trim();

    // Create inventory HashMap
    let mut inventory: HashMap<String, (f64, i32)> = HashMap::new();

    // Parse comma-separated inputs
    let names: Vec<String> = items_input
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

    // Populate inventory
    for i in 0..names.len() {
        inventory.insert(names[i].clone(), (prices[i], quantities[i]));
    }

    // Handle commands
    match command {
        "add" => {
            let price: f64 = value_input.parse().unwrap();

            let (item_price, item_quantity) = inventory
                .entry(item_name.clone())
                .or_insert((price, 0));

            println!("Added {} to inventory", item_name);
            println!("Price: ${}", item_price);
            println!("Quantity: {}", item_quantity);
        }

        "check" => {
            match inventory.get(&item_name) {
                Some((item_price, item_quantity)) => {
                    println!("{} is in stock", item_name);
                    println!("Price: ${}", item_price);
                    println!("Quantity: {}", item_quantity);
                }
                None => {
                    println!("{} is not in stock", item_name);
                }
            }
        }

        "sell" => {
            let quantity_to_sell: i32 = value_input.parse().unwrap();

            match inventory.get(&item_name) {
                Some((item_price, item_quantity)) => {
                    if *item_quantity >= quantity_to_sell && quantity_to_sell > 0 {
                        let total_price = *item_price * quantity_to_sell as f64;
                        let new_quantity = *item_quantity - quantity_to_sell;

                        inventory.insert(
                            item_name.clone(),
                            (*item_price, new_quantity),
                        );

                        println!("Sold {} {}", quantity_to_sell, item_name);
                        println!("Total: ${}", total_price);
                        println!("Remaining stock: {}", new_quantity);
                    } else {
                        println!("Insufficient stock for {}", item_name);
                        println!("Available: {}", item_quantity);
                    }
                }
                None => {
                    println!("{} is not in stock", item_name);
                }
            }
        }

        _ => {
            println!("Invalid command");
        }
    }
}