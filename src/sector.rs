use crate::generation::GenParams;
use crate::helpers::SurfaceLocator;
use crate::star::Star;
use std::option::Option;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Sector {
    x: i32,
    y: i32,
    #[serde(rename = "numStars")]
    num_stars: u32,
    #[serde(skip)]
    generated: bool,
    stars: Vec<Star>,
}

impl Sector {
    pub fn new(x: i32, y: i32, gen: &GenParams) -> Self {
        let num_stars = gen.sector.num_stars.gen_rand();
        let mut stars: Vec<Star> = Vec::new();
        let loc = SurfaceLocator {
            planet_pos: 0,
            star_pos: 0,
            sec_x: x,
            sec_y: y,
        };
        for i in 0..num_stars {
            let mut star_loc = loc;
            star_loc.star_pos = i as usize;
            stars.push(Star::new(gen, star_loc));
        }
        Self {
            x,
            y,
            num_stars,
            generated: false,
            stars,
        }
    }

    pub fn generate(&mut self) {
        self.generated = true;
    }
    pub fn get_star_mut(&mut self, pos: usize) -> Option<&mut Star> {
        self.stars.get_mut(pos)
    }
}
