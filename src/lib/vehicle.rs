// TODO: Define a public trait called Rentable
// It should have a method rental_info that takes &self and returns a String
pub trait Rentable {
    fn rental_info(&self) -> String;
}

// TODO: Define a public struct Car with a public field 'model' of type String
// Implement the Rentable trait for Car
// rental_info should return "Car: {model}"
pub struct Car {
    pub model: String
}

impl Rentable for Car {
    fn rental_info(&self) -> String {
        format!("Car: {}",self.model)
    }
}

// TODO: Define a public struct Bike with a public field 'brand' of type String
// Implement the Rentable trait for Bike
// rental_info should return "Bike: {brand}"

pub struct Bike {
    pub brand: String
}

impl Rentable for Bike {
    fn rental_info(&self) -> String {
        format!("Bike: {}",self.brand)
    }
}