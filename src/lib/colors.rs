// TODO: Derive PartialEq and Eq traits for the Color struct
// This will allow comparing two Color instances using ==

// TODO: Define a public Color struct with three public fields:
// - red: u8
// - green: u8
// - blue: u8
#[derive(PartialEq, Eq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}