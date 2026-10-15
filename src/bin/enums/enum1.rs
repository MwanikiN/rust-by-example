mod traffic;

use traffic::TrafficLight;

fn main() {
    // Read input for durations
    let mut red_input = String::new();
    std::io::stdin().read_line(&mut red_input).expect("Failed to read line");
    let red_duration: u32 = red_input.trim().parse().expect("Invalid number");

    let mut yellow_input = String::new();
    std::io::stdin().read_line(&mut yellow_input).expect("Failed to read line");
    let yellow_duration: u32 = yellow_input.trim().parse().expect("Invalid number");

    let mut green_input = String::new();
    std::io::stdin().read_line(&mut green_input).expect("Failed to read line");
    let green_duration: u32 = green_input.trim().parse().expect("Invalid number");

    // TODO: Create instances of each TrafficLight variant with the input durations
    let red: TrafficLight = TrafficLight::Red(red_duration);
    let yellow: TrafficLight = TrafficLight::Yellow(yellow_duration);
    let green: TrafficLight = TrafficLight::Green(green_duration);
    // TODO: Call the status method on each light and print the result
    println!("{}",red.status());
    println!("{}",yellow.status());
    println!("{}",green.status());
}
