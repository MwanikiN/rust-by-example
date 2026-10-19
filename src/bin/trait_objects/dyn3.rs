// mod items;

use rust_by_example::lib::items::{Priceable, Product, Service};

fn main() {
    // Read inputs
    let mut product_name = String::new();
    std::io::stdin().read_line(&mut product_name).expect("Failed to read line");
    let product_name = product_name.trim().to_string();

    let mut product_cost = String::new();
    std::io::stdin().read_line(&mut product_cost).expect("Failed to read line");
    let product_cost: f64 = product_cost.trim().parse().expect("Failed to parse");

    let mut service_name = String::new();
    std::io::stdin().read_line(&mut service_name).expect("Failed to read line");
    let service_name = service_name.trim().to_string();

    let mut hourly_rate = String::new();
    std::io::stdin().read_line(&mut hourly_rate).expect("Failed to read line");
    let hourly_rate: f64 = hourly_rate.trim().parse().expect("Failed to parse");

    let mut hours = String::new();
    std::io::stdin().read_line(&mut hours).expect("Failed to read line");
    let hours: f64 = hours.trim().parse().expect("Failed to parse");

    // TODO: Create a Product using product_name and product_cost
    let product: Box<dyn Priceable> = Box::new(Product {name: product_name, cost: product_cost});
    // TODO: Create a Service using service_name, hourly_rate, and hours
    let service: Box<dyn Priceable> = Box::new(Service {name: service_name, hourly_rate, hours});
    // TODO: Create a Vec<Box<dyn Priceable>> and add both items to it
    let mut prices: Vec<Box<dyn Priceable>> = vec![];
    // TODO: Iterate through the vector and calculate the total price
    prices.push(product);
    prices.push(service);
    // TODO: Print the total with one decimal place
    // Format: Total: ${total:.1}
    let mut total = 0.0;
    for cost in &prices{
        total += cost.price();
    }
    println!("Total: ${:.1}",total);
}
