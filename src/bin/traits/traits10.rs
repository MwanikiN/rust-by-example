// mod printable;

use rust_by_example::lib::printable::{Printable, announce};

// TODO: Define a public struct Person with a public name field (String)
pub struct Person {
    pub name: String
}
// Implement the Printable trait for Person
// print_info should return "Person: {name}"
impl Printable for Person {
    fn print_info(&self) -> String {
        format!("Person: {}",self.name)
    }
}
// TODO: Define a public struct Product with public name (String) and price (f64) fields
// Implement the Printable trait for Product
// print_info should return "Product: {name} - ${price}"
pub struct Product {
    pub name: String,
    pub price: f64
}

impl Printable for Product{
    fn print_info(&self) -> String {
        format!("Product: {} - ${}",self.name, self.price)
    }
}

fn main() {
    // Read inputs
    let mut person_name = String::new();
    std::io::stdin().read_line(&mut person_name).expect("Failed to read line");
    let person_name = person_name.trim().to_string();

    let mut product_name = String::new();
    std::io::stdin().read_line(&mut product_name).expect("Failed to read line");
    let product_name = product_name.trim().to_string();

    let mut price_input = String::new();
    std::io::stdin().read_line(&mut price_input).expect("Failed to read line");
    let price: f64 = price_input.trim().parse().expect("Failed to parse price");

    // TODO: Create a Person instance with person_name
    let person: Person = Person {name: person_name};
    // TODO: Create a Product instance with product_name and price
    let product: Product = Product {name: product_name, price};
    // TODO: Call announce with the Person
    announce(person);
    // TODO: Call announce with the Product
    announce(product);
}
