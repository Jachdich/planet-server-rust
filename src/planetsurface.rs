use crate::helpers::SurfaceLocator;
use crate::task::Task;
use crate::tile::Tile;
use common::resources::Resources;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PlanetSurface {
    tiles: Vec<Tile>,
    pub tasks: Vec<Task>,
    pub generated: bool,

    resources: Resources,
    rad: u32,
    noise_scale: f64,
    noise_z: f64,
    last_ticks: u64,
    loc: SurfaceLocator,
}

impl PlanetSurface {
    pub fn generate(loc: SurfaceLocator) -> Self {
        Self {
            rad: 0,
            generated: false,
            noise_scale: 0.0,
            noise_z: 0.0,
            last_ticks: 0,
            tiles: Vec::new(),
            tasks: Vec::new(),
            resources: Resources::new(),
            loc,
        }
    }
}
