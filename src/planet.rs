use crate::planetsurface::PlanetSurface;
use crate::helpers::Colour;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Planet {
    rad: u32,
    sea_level: i32,
    num_colours: u32,
    owner: u64,

    gen_chances: Vec<f64>,
    gen_colours: Vec<Colour>,
    gen_zvalues: Vec<i32>,
    gen_noise:   Vec<f64>,

    base_colour: Colour,
    pos_from_star: u32,
    angular_velocity: f64,
    theta: f64,
    
    sector_seed: u32,
    surface: PlanetSurface,
}
