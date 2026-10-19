// TODO: Define GridPoint struct
// - Should have public fields: x (i32) and y (i32)
// - Derive Copy, Clone (and Debug if needed)
// Since it only contains primitive types, it can implement Copy
#[derive(Copy, Clone)]
pub struct GridPoint {
    pub x: i32,
    pub y: i32
}

// TODO: Define NamedLocation struct
// - Should have public fields: name (String), x (i32), y (i32)
// - Derive Clone (and Debug if needed)
// Note: Cannot derive Copy because String doesn't implement Copy
#[derive(Clone)]
pub struct NamedLocation {
    pub name: String,
    pub x: i32,
    pub y: i32
}
