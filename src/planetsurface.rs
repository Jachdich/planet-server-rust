use crate::generation::{GenParams, Range};
use crate::helpers::{Colour, SurfaceLocator};
use crate::planet::Planet;
use crate::task::Task;
use crate::tile::{Tile, TileType};
use common::resources::Resources;
use noise::{NoiseFn, Perlin, Seedable, Fbm};
use rand::Rng;
use std::default::Default;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PlanetSurface {
    tiles: Vec<Tile>,
    pub tasks: Vec<Task>,

    resources: Resources,
    rad: u32,
    #[serde(skip)]
    population_noise_scale: f64,
    #[serde(skip)]
    population_noise_seed: i32,
    #[serde(skip)]
    height_noise_scale: f64,
    #[serde(skip)]
    height_noise_seed: i32,
    #[serde(skip)]
    last_ticks: u64,
    #[serde(skip)]
    locator: SurfaceLocator,
    #[serde(skip)]
    sea_level: i32,
    #[serde(skip)]
    gen_scales: Vec<f64>,
    #[serde(skip)]
    gen_seeds: Vec<i32>,
    #[serde(skip)]
    gen_thresholds: Vec<f64>,
    #[serde(skip)]
    gen_colours: Vec<Colour>,
    #[serde(skip)]
    base_colour: Colour,
    #[serde(skip)]
    num_colours: u32,
}

impl PlanetSurface {
    fn get_type_at(&self, x: u32, y: u32, z: i32, col: Colour) -> TileType {
        let mut rng = rand::thread_rng();
        if z <= self.sea_level {
            TileType::Water
        } else {
            if col.g() > col.r() && col.g() as f64 > col.b() as f64 * 1.5 {
                if rng.gen_range(0..3) == 0 {
                    TileType::Grass
                } else {
                    let noise_gen = Fbm::<Perlin>::default().set_seed(self.population_noise_seed as u32);
                    let noise = (noise_gen.get([
                        x as f64 / self.population_noise_scale,
                        y as f64 / self.population_noise_scale,
                    ]) + 1.0) / 2.0;

                    if noise < 0.2 { TileType::Grass }
                    else if noise < 0.3 { TileType::Bush }
                    else if noise < 0.5 { TileType::Tree }
                    else if noise < 0.7 { TileType::Forest }
                    else if noise < 0.8 { TileType::Pine }
                    else { TileType::Pineforest }
                }
            } else {
                TileType::Grass
            }
        }
    }

    fn get_height_at(&self, x: u32, y: u32) -> i32 {
        let xb: f64 = x as f64 - self.rad as f64;
        let yb: f64 = y as f64 - self.rad as f64;
        let noise_gen = Fbm::<Perlin>::default().set_seed(self.height_noise_seed as u32);
        
        let noise = noise_gen.get([
            xb / self.height_noise_scale,
            yb / self.height_noise_scale,
        ]);
        let height = (noise * 15.0) as i32;
        if height < self.sea_level {
            self.sea_level
        } else {
            height
        }
    }

    fn get_colour_at(&self, x: u32, y: u32) -> Colour {
        let x = (x as f64) - self.rad as f64;
        let y = (y as f64) - self.rad as f64;
        let mut r: u32 = 0;
        let mut g: u32 = 0;
        let mut b: u32 = 0;
        let mut total: u32 = 0;
        for i in 1..self.num_colours {
            let noise_gen = Fbm::<Perlin>::default().set_seed(self.gen_seeds[i as usize] as u32);
            let noise = noise_gen.get([x / self.gen_scales[i as usize], y / self.gen_scales[i as usize]]);
            if noise > self.gen_thresholds[i as usize] {
                r += self.gen_colours[i as usize].r() as u32;
                g += self.gen_colours[i as usize].g() as u32;
                b += self.gen_colours[i as usize].b() as u32;
                total += 1;
            }
        }
        if total == 0 {
            self.base_colour
        } else {
            Colour::new_rgb((r / total) as u8, (g / total) as u8, (b / total) as u8)
        }
    }

    fn get_tile(&self, x: u32, y: u32) -> Tile {
        let z = self.get_height_at(x, y);
        let colour = self.get_colour_at(x, y);
        Tile {
            ty: self.get_type_at(x, y, z, colour),
            z,
            colour,
        }
    }

    pub fn new() -> Self {
        Self {
            tiles: Vec::new(),
            tasks: Vec::new(),
            resources: Resources::new(),
            rad: 0,
            population_noise_scale: 0.0,
            population_noise_seed: 0,
            last_ticks: 0,
            locator: SurfaceLocator::new(),
            sea_level: 0,
            height_noise_scale: 0.0,
            height_noise_seed: 0,
            gen_scales: Vec::new(),
            gen_seeds: Vec::new(),
            gen_thresholds: Vec::new(),
            gen_colours: Vec::new(),
            base_colour: Default::default(),
            num_colours: 0,
        }
    }

    pub fn generate(&mut self, gen: &GenParams, rad: u32, locator: SurfaceLocator) {
        let mut rng = rand::thread_rng();
        self.rad = rad;
        self.locator = locator;
        self.sea_level = gen.planet.sea_level.gen_rand();
        self.num_colours = gen.planet.num_colours.gen_rand();
        for _ in 0..self.num_colours {
            self.gen_thresholds.push(gen.planet.gen_chance.gen_rand());
            self.gen_scales.push(gen.planet.gen_noise.gen_rand());
            self.gen_seeds.push(rng.gen_range(0..1000000));
            self.gen_colours.push(Colour::rand(&Range::new(0..=255)));
        }
        self.height_noise_scale = self.gen_scales[0];
        self.height_noise_seed = self.gen_seeds[0];

        self.population_noise_scale = gen.planet.gen_noise_plants.gen_rand();
        self.population_noise_seed = rng.gen_range(0..10000000);
        

        self.base_colour = Colour::rand(&gen.planet.base_colour);

        for y in 0..rad * 2 {
            for x in 0..rad * 2 {
                self.tiles.push(self.get_tile(x, y));
            }
        }
    }
}
