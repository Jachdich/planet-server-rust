use crate::star::Star;
use crate::generation::GenParams;
use nanorand::{Rng, WyRand};

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
        let mut rng = WyRand::new_seed(69);
        let num_stars = gen.sector.num_stars.gen_rand(&mut rng);
        let mut stars: Vec<Star> = Vec::new();
        for _ in 0..num_stars {
            stars.push(Star::new(gen, &mut rng));
        }
        Self {
            x, y,
            num_stars,
            generated: false,
            stars: stars
        }
    }

    pub fn generate(&mut self) {
        self.generated = true;
    }
}
