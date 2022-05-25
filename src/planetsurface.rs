use crate::helpers::SurfaceLocator;
use crate::tile::Tile;
use crate::task::Task;
use common::resources::Resources;

pub struct PlanetSurface {
    tiles: Vec<Tile>,
    tasks: Vec<Task>,
    generated: bool,
    resources: Resources,
    rad: u32,
    noise_scale: f64,
    noise_z: f64,
    last_ticks: u64,
    loc: SurfaceLocator,
}
