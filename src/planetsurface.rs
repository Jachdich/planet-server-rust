use crate::helpers::SurfaceLocator;
use crate::task::Task;
use crate::tile::{Tile, TileType};
use common::resources::Resources;
use crate::planet::Planet;
use noise::{Seedable, Simplex};
use rand::Rng;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PlanetSurface {
    tiles: Vec<Tile>,
    pub tasks: Vec<Task>,

    resources: Resources,
    rad: u32,
    #[serde(skip)]
    noise_scale: f64,
    #[serde(skip)]
    noise_z: f64,
    #[serde(skip)]
    last_ticks: u64,
    #[serde(skip)]
    locator: SurfaceLocator,
}

impl PlanetSurface {
    pub fn generate(parent: &Planet) -> Self {
        let mut tiles = Vec::new();
        let mut rng = rand::thread_rng();
        let rad = parent.radius;
        for y in 0..rad * 2 {
            for x in 0..rad * 2 {
                tiles.push(Tile { ty: if rng.gen_range(0..0xF) == 0x8 { TileType::Tree } else { TileType::Grass }, z: 0 });
            }
        }
        Self {
            rad,
            noise_scale: 0.0,
            noise_z: 0.0,
            last_ticks: 0,
            tiles,
            tasks: Vec::new(),
            resources: Resources::new(),
            locator: parent.locator,
        }
    }
}
