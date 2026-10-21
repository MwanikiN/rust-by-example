// TODO: Define a public Memory struct with one public field:
// - size_gb: u32
pub struct Memory {
    pub size_gb: u32
}
// TODO: Implement a specs method on Memory that returns a String
// Format: "{size_gb}GB RAM"
impl Memory {
    pub fn specs(&self) -> String {
        format!("{}GB RAM",self.size_gb)
    }
}