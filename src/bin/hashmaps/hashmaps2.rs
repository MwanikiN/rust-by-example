use std::collections::HashMap;
use std::io;

fn main() {
    // Read the country name to insert
    let mut country = String::new();
    io::stdin().read_line(&mut country).expect("Failed to read line");
    let country = country.trim().to_string();
    
    // Read the capital city name
    let mut capital = String::new();
    io::stdin().read_line(&mut capital).expect("Failed to read line");
    let capital = capital.trim().to_string();
    
    // Read the country name to look up
    let mut lookup_country = String::new();
    io::stdin().read_line(&mut lookup_country).expect("Failed to read line");
    let lookup_country = lookup_country.trim().to_string();
    
    // TODO: Write your code below
    // Create a HashMap, insert the country and capital, then look up the country
    let mut capital_cities: HashMap<String, String> = HashMap::new();
    capital_cities.insert(country, capital);

    match capital_cities.get(&lookup_country) {
        Some(capital) => println!("The capital of {} is {}",&lookup_country, capital),
        None => println!("{} not found in the map", &lookup_country)
    }

    
}