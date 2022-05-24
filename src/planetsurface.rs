pub struct PlanetSurface {
    tiles: Vec<Tile>,
    tasks: Vec<Task>,
    generated: bool,
    resources: Resources,
    rad: u32,
    noise_scale: f64,
    noise_z: f64,
    last_ticks: u64,
    SurfaceLocator loc,
}
