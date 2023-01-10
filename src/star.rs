use crate::generation::GenParams;
use crate::helpers::{Colour, SurfaceLocator};
use crate::planet::Planet;
use rand::Rng;
use std::option::Option;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Star {
    x: u32,
    y: u32,
    radius: u32,
    colour: Colour,
    #[serde(rename = "num")]
    num_planets: usize,
    planets: Option<Vec<Planet>>,

    #[serde(rename = "noiseZ")]
    noise_z: f64,
    #[serde(rename = "noiseScl")]
    noise_scl: f64,
    #[serde(rename = "noiseEffect")]
    noise_effect: f64,

    effective_owner: u64,
}

//This is not my function lol
//TODO Find a better function/modify this to be more rusty
fn k_to_rgb(k: u32) -> u32 {
    let temp: f64 = k as f64 / 100.0;
    let mut r: f64;
    let mut g: f64;
    let mut b: f64;

    if temp <= 66.0 {
        r = 255.0;
    } else {
        r = temp - 60.0;
        r = 329.698727446 * r.powf(-0.1332047592);
        if r < 0.0 {
            r = 0.0;
        }
        if r > 255.0 {
            r = 255.0;
        }
    }

    if temp <= 66.0 {
        g = temp;
        g = 99.4708025861 * g.ln() - 161.1195681661;
        if g < 0.0 {
            g = 0.0;
        }
        if g > 255.0 {
            g = 255.0;
        }
    } else {
        g = temp - 60.0;
        g = 288.1221695283 * g.powf(-0.0755148492);
        if g < 0.0 {
            g = 0.0;
        }
        if g > 255.0 {
            g = 255.0;
        }
    }

    if temp >= 66.0 {
        b = 255.0;
    } else {
        if temp <= 19.0 {
            b = 0.0;
        } else {
            b = temp - 10.0;
            b = 138.5177312231 * b.ln() - 305.0447927307;
            if b < 0.0 {
                b = 0.0;
            }
            if b > 255.0 {
                b = 255.0;
            }
        }
    }

    return ((r as u32) & 0xff) << 16 | ((g as u32) & 0xff) << 8 | ((b as u32) & 0xff);
}

impl Star {
    pub fn new(gen: &GenParams) -> Self {
        let mut rng = rand::thread_rng();
        let radius = gen.star.rad.gen_rand();
        Self {
            x: rng.gen_range(0..256),
            y: rng.gen_range(0..256),
            radius,
            colour: Colour::new_u32(k_to_rgb(gen.star.temp.gen_rand())),
            num_planets: gen.star.num_planets.gen_rand() as usize,
            planets: None,
            noise_z: rng.gen_range(0.0..100000.0),
            noise_scl: gen.star.noise_scl.gen_rand(),
            noise_effect: gen.star.noise_effect.gen_rand(),

            effective_owner: 0,
        }
    }

    pub fn gen_planets(&mut self, gen: &GenParams, loc: SurfaceLocator) -> &Vec<Planet> {
        let mut rng = rand::thread_rng();
        let mut planets: Vec<Planet> = Vec::new();
        let mut last_dist: u32 = rng.gen_range(0..100) + self.radius * 6 + 20;
        for i in 0..self.num_planets {
            let mut planet_loc = loc;
            planet_loc.planet_pos = i as usize;
            planets.push(Planet::new(gen, last_dist, loc));
            last_dist += planets.last().unwrap().radius * 2 + rng.gen_range(0..100);
        }
        self.planets = Some(planets);
        self.planets.as_ref().unwrap()
    }

    pub fn get_planet_mut(&mut self, pos: usize) -> Option<&mut Planet> {
        self.planets.as_mut()?.get_mut(pos)
    }
}
