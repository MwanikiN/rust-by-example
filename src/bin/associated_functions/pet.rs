
pub struct Pet {
    name: String,
    energy: u32,
    hunger: u32,
}

impl Pet {
    pub fn new(name: String) -> Self {
        Pet {
            name,
            energy: 100,
            hunger: 0,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    // Set the initial hunger level
    pub fn set_hunger_energy(&mut self, energy: u32, hunger: u32) {
        self.energy = energy;
        self.hunger = hunger;
    }

//     // Feed the pet and reduce hunger by 30
//     pub fn play(&mut self) {
//     self.hunger = self.hunger.saturating_add(15).min(100);
//     self.energy = self.energy.saturating_sub(20);
// }
     // Return the current energy level
    pub fn energy(&self) -> u32 {
        self.energy
    }

    // Return the current hunger level
    pub fn hunger(&self) -> u32 {
        self.hunger
    }
    
    //fn status of the pet
    pub fn status(&self) -> String {
        if self.energy >= 50 && self.hunger <= 50 {
            String::from("happy")
        }
        else {
            String::from("tired")
        }
    }
}
