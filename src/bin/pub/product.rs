// TODO: Define a public Product struct with two fields:
// - name (String)
// - price (f64)
// Remember to make the struct and its fields public
pub struct Product {
    pub name: String,
    pub price: f64
}


// TODO: Implement an associated function called 'new' for Product
// - It should take a name (String) and price (f64) as parameters
// - It should return a new Product instance
// - Remember to make the function public
impl Product {
    pub fn new(name: String, price: f64) -> Product {
        Product {
            name,
            price
        }
    }
}