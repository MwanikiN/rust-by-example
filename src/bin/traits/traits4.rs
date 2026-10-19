mod prod;

use prod::Product;

fn main() {
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim().to_string();

    let mut price_input = String::new();
    std::io::stdin().read_line(&mut price_input).expect("Failed to read line");
    let price: f64 = price_input.trim().parse().expect("Failed to parse price");

    // TODO: Create a Product instance with the name and price
    let product: Product = Product {name, price};
    // TODO: Print the product using the {} formatter
    println!("{}",product);
}
