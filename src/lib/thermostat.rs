// TODO: Define a public Thermostat struct with a private temperature field (i32)
pub struct Thermostat {
    temperature: i32
}

impl Thermostat {
    // TODO: Implement a 'new' constructor that takes an initial temperature
    pub fn new(temperature: i32) -> Self {
        // TODO: Create and return a new Thermostat
        Thermostat {
            temperature
        }
    }

    // TODO: Implement a 'temperature' getter that returns the current temperature
    pub fn temperature(&self) -> i32 {
        // TODO: Return the temperature
        self.temperature
    }

    // TODO: Implement a 'set_temperature' setter
    // Only accept values between 10 and 30 (inclusive)
    // If the value is outside this range, keep the temperature unchanged
    pub fn set_temperature(&mut self, temp: i32) {
        // TODO: Validate and set the temperature
        if temp >=10 && temp <= 30 {
            self.temperature = temp;
        }
    }
}
