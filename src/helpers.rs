use crate::generation::Range;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Colour(u32);

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SurfaceLocator;


impl Colour {
    pub fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        Self{0: ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)}
    }

    pub fn new_u32(n: u32) -> Self {
        Self{0: n}
    }

    pub fn rand(range: &Range<u8>) -> Self {
        Colour::new_rgb(range.gen_rand(), range.gen_rand(), range.gen_rand())
    }

    pub fn r(&self) -> u8 { ((self.0 >> 16) & 0xFF) as u8 }
    pub fn g(&self) -> u8 { ((self.0 >> 8) & 0xFF) as u8 }
    pub fn b(&self) -> u8 { (self.0 & 0xFF) as u8 }
}
