
pub struct Sector {
    x: u32,
    y: u32,
    rad: u32,
    num_stars: u32,
    generated: bool,
    stars: Vec<Star>,
}
