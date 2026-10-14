// Declare the product module
mod product;

// TODO: Bring Product into scope with a use statement
use product::Product;

fn main() {
    // Read input
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim().to_string();
    
    let mut price_input = String::new();
    std::io::stdin().read_line(&mut price_input).expect("Failed to read line");
    let price: f64 = price_input.trim().parse().expect("Failed to parse price");
    
    // TODO: Create a new Product using the Product::new function
    let prod: Product = Product::new(name,price);
    // TODO: Print the product details in the format:
    // Product: {name}, Price: ${price}
    println!("Product: {}, Price: ${}",prod.name, prod.price);
}
