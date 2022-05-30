use crate::planetsurface::PlanetSurface;
use crate::helpers::Colour;
use crate::generation::GenParams;
use crate::generation::Range;
use rand::Rng;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Planet {
    pub radius: u32,
    sea_level: i32,
    num_colours: u32,
    owner: u64,

    #[serde(rename = "generationChances")]
    gen_chances: Vec<f64>,
    #[serde(rename = "generationColours")]
    gen_colours: Vec<Colour>,
    #[serde(rename = "generationZValues")]
    gen_zvalues: Vec<i32>,
    #[serde(rename = "generationNoise")]
    gen_noise:   Vec<f64>,

    base_colour: Colour,
    pos_from_star: u32,
    angular_velocity: f64,
    theta: f64,
    
    sector_seed: u32,
    surface: PlanetSurface,
}

impl Planet {
    pub fn new(gen: &GenParams, pos_from_star: u32) -> Self {
        let mut rng = rand::thread_rng();
        
        let num_colours = gen.planet.num_colours.gen_rand();
        let mut gen_chances = Vec::new();
        let mut gen_colours = Vec::new();
        let mut gen_zvalues = Vec::new();
        let mut gen_noise = Vec::new();

        for _ in 0..num_colours {
            gen_chances.push(gen.planet.gen_chance.gen_rand());
            gen_noise.push(gen.planet.gen_noise.gen_rand());
            gen_zvalues.push(rng.gen_range(0..1000000));
            gen_colours.push(Colour::rand(&Range::new(0..=255)));
        }
        
        Self {
            radius: gen.planet.rad.gen_rand(),
            sea_level: gen.planet.sea_level.gen_rand(),
            num_colours,
            owner: 0,
            gen_chances,
            gen_colours,
            gen_zvalues,
            gen_noise,

            base_colour: Colour::rand(&gen.planet.base_colour),
            pos_from_star,
            angular_velocity: 1.0 / ((pos_from_star * pos_from_star) as f64) * gen.planet.angular_velocity_mult,
            theta: rng.gen_range(0.0..(2.0*3.14159265358979323)),
            sector_seed: 0,
            surface: PlanetSurface::new(),
        }
    }
}
