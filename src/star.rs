use crate::planet::Planet;
use crate::helpers::Colour;
use crate::generation::GenParams;
use rand::Rng;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Star {
    x: u32,
    y: u32,
    radius: u32,
    colour: Colour,
    #[serde(rename = "num")]
    num_planets: usize,
    planets: Vec<Planet>,

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
    let mut r: i16;
    let mut g: i16;
    let mut b: i16;

    if temp <= 66.0 {
    	r = 255;
    } else {
    	r = (temp - 60.0) as i16;
    	r = (329.698727446 * (r as f64).powf(-0.1332047592)) as i16;
    	if r < 0 { r = 0; }
    	if r > 255 { r = 255; }
    }
        
    if temp <= 66.0 {
    	g = temp as i16;
    	g = (99.4708025861 * f64::ln(g as f64) - 161.1195681661) as i16;
    	if g < 0 { g = 0; }
    	if g > 255 { g = 255; }
    } else {
    	g = (temp - 60.0) as i16;
    	g = (288.1221695283 * (g as f64).powf(-0.0755148492)) as i16;
    	if g < 0 { g = 0; }
    	if g > 255 { g = 255; }
    }
    
    if temp >= 66.0 {
    	b = 255;
    } else {
    	if temp <= 19.0 {
    		b = 0;
    	} else {
    		b = (temp - 10.0) as i16;
    		b = (138.5177312231 * f64::ln(b as f64) - 305.0447927307) as i16;
    		if b < 0 { b = 0; }
    		if b > 255 { b = 255; }
    	}
    }

    return ((r as u32) & 0xff) << 16 | ((g as u32) & 0xff) << 8 | ((b as u32) & 0xff);
}


impl Star {
    pub fn new(gen: &GenParams) -> Self {
        let mut planets: Vec<Planet> = Vec::new();
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(0..256),
            y: rng.gen_range(0..256),
            radius: gen.star.rad.gen_rand(),
            colour: Colour::new_u32(k_to_rgb(gen.star.temp.gen_rand())),
            num_planets: planets.len(),
            planets,
            noise_z: rng.gen_range(0.0..100000.0),
            noise_scl: gen.star.noise_scl.gen_rand(),
            noise_effect: gen.star.noise_effect.gen_rand(),

            effective_owner: 0
        }
    }
}
