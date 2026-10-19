use std::fmt;

// TODO: Define a public Product struct with public fields:
// - name: String
// - price: f64
pub struct Product {
    pub name: String,
    pub price: f64
}

// TODO: Implement the Display trait for Product
// The format should be: {name} - ${price}
// Use the write! macro inside the fmt method
impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - ${}",self.name, self.price)
    }
}