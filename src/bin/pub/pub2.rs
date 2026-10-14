mod thermostat;

use thermostat::Thermostat;

fn main() {
    // Read inputs
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Failed to read line");
    let initial_temp: i32 = input1.trim().parse().expect("Invalid number");

    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Failed to read line");
    let first_attempt: i32 = input2.trim().parse().expect("Invalid number");

    let mut input3 = String::new();
    std::io::stdin().read_line(&mut input3).expect("Failed to read line");
    let second_attempt: i32 = input3.trim().parse().expect("Invalid number");

    // TODO: Create a thermostat with the initial temperature
    let mut thermostat: Thermostat = Thermostat::new(initial_temp);
    // TODO: Print the initial temperature
    // Format: "Initial: {temperature}"
    println!("Initial: {}", thermostat.temperature());

    // TODO: Attempt to set the first temperature and print the result
    // Format: "After setting to {attempted_value}: {temperature}"
    thermostat.set_temperature(first_attempt);
    println!("After setting to {}: {}", first_attempt, thermostat.temperature());

    // TODO: Attempt to set the second temperature and print the result
    // Format: "After setting to {attempted_value}: {temperature}"
    thermostat.set_temperature(second_attempt);
    println!("After setting to {}: {}", second_attempt, thermostat.temperature());
}
