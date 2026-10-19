// TODO: Define a public trait called Priceable
// It should have a method called price that takes &self and returns f64
pub trait Priceable {
    fn price(&self) -> f64;
}

// TODO: Define a public struct called Product
// It should have public fields: name (String) and cost (f64)
pub struct Product {
    pub name: String,
    pub cost: f64
}

// TODO: Implement the Priceable trait for Product
// The price method should return the cost
impl Priceable for Product {
    fn price(&self) -> f64 {
        self.cost
    }
}

// TODO: Define a public struct called Service
// It should have public fields: name (String), hourly_rate (f64), and hours (f64)
pub struct Service {
    pub name: String,
    pub hourly_rate: f64,
    pub hours: f64
}

// TODO: Implement the Priceable trait for Service
// The price method should return hourly_rate * hours
impl Priceable for Service {
    fn price(&self) -> f64 {
        self.hourly_rate * self.hours
    }
}
