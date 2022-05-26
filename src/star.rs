use crate::planet::Planet;
use crate::helpers::Colour;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Star {
    x: u32,
    y: u32,
    rad: u32,
    colour: Colour,
    planets: Vec<Planet>,
    
    noise_z: f64,
    noise_scl: f64,
    noise_effect: f64,

    effective_owner: u64,
}
