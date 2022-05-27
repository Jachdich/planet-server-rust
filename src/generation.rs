use std::ops::RangeInclusive;
use serde::Serialize;
use serde::Serializer;
use serde::Deserialize;
use serde::Deserializer;

pub struct Range<T>(RangeInclusive<T>);

impl<T> Serialize for Range<T> where T: Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
        where S: Serializer {
        serializer::Ok
    }
}

impl<'a, T> Deserialize<'a> for Range<T> where T: Deserialize<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> 
        where D: Deserializer<'a> {
        
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct StarGen {
    pub num_planets: Range<u32>,
    pub rad: Range<u32>,
    pub temp: Range<u32>,
    pub noise_scl: Range<f64>,
    pub noise_effect: Range<f64>
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct PlanetGen {
    pub rad: Range<u32>,
    pub gen_chance: Range<f64>,
    pub gen_noise: Range<f64>,
    pub gen_noise_plants: Range<f64>,
    pub base_colour: Range<u8>,
    pub angular_velocity_mut: f64,
    pub num_colours: Range<u32>,
    pub sea_level: Range<i32>
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SectorGen {
    pub num_stars: Range<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenParams {
    pub planet: PlanetGen,
    pub sector: SectorGen,
    pub star: StarGen,
    pub level_seed: i32
}

