use crate::generation::Range;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Colour(u32);

#[derive(serde::Serialize, serde::Deserialize, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceLocator {
    pub planet_pos: usize,
    pub star_pos: usize,
    pub sec_x: i32,
    pub sec_y: i32,
}

impl SurfaceLocator {
    // pub fn new() -> Self {
    //     Self {
    //         planet_pos: 0,
    //         star_pos: 0,
    //         sec_x: 0,
    //         sec_y: 0,
    //     }
    // }
}

impl Colour {
    pub fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            0: ((r as u32) << 16) | ((g as u32) << 8) | (b as u32),
        }
    }

    pub fn new_u32(n: u32) -> Self {
        Self { 0: n }
    }

    pub fn rand(range: &Range<u8>) -> Self {
        Colour::new_rgb(range.gen_rand(), range.gen_rand(), range.gen_rand())
    }

    pub fn r(&self) -> u8 {
        ((self.0 >> 16) & 0xFF) as u8
    }
    pub fn g(&self) -> u8 {
        ((self.0 >> 8) & 0xFF) as u8
    }
    pub fn b(&self) -> u8 {
        (self.0 & 0xFF) as u8
    }
}
