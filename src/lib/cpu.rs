// TODO: Define a public CPU struct with two public fields:
// - cores: u32
// - speed_ghz: f32
pub struct CPU {
    pub cores: u32,
    pub speed_ghz: f32
}

// TODO: Implement a specs method on CPU that returns a String
// Format: "{cores}-core @ {speed_ghz}GHz"
impl CPU {
    pub fn specs(&self) -> String {
        format!("{}-core @ {}GHz",self.cores, self.speed_ghz)
    }
}