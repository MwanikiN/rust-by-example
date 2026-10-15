// TODO: Define a public TrafficLight enum with three variants:
// - Red(u32)
// - Yellow(u32)
// - Green(u32)
pub enum TrafficLight {
    Red(u32),
    Yellow(u32),
    Green(u32)
}
// TODO: Implement the TrafficLight enum with a status method
// The status method should return a String with the appropriate message:
// - Red: "Stop for {seconds} seconds"
// - Yellow: "Caution for {seconds} seconds"
// - Green: "Go for {seconds} seconds"
// Use match to handle each variant
impl TrafficLight {
    pub fn status(&self) -> String {
        match self {
            TrafficLight::Red(sec) => format!("Stop for {} seconds",sec),
            TrafficLight::Yellow(sec) => format!("Caution for {} seconds", sec),
            TrafficLight::Green(sec) => format!("Go for {} seconds", sec)
        }
    }
}