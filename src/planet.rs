use crate::generation::GenParams;
use crate::generation::Range;
use crate::helpers::Colour;
use crate::helpers::SurfaceLocator;
use crate::planetsurface::PlanetSurface;
use rand::Rng;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Planet {
    pub radius: u32,
    #[serde(skip)]
    owner: u64,

    pos_from_star: u32,
    angular_velocity: f64,
    theta: f64,

    sector_seed: u32,
    pub surface: PlanetSurface,
    pub locator: SurfaceLocator,
}

impl Planet {
    pub fn new(gen: &GenParams, pos_from_star: u32, locator: SurfaceLocator) -> Self {
        let mut rng = rand::thread_rng();
        let mut this = Self {
            radius: gen.planet.rad.gen_rand(),
            owner: 0,

            pos_from_star,
            angular_velocity: 1.0 / ((pos_from_star * pos_from_star) as f64)
                * gen.planet.angular_velocity_mult,
            theta: rng.gen_range(0.0..(2.0 * 3.14159265358979323)),
            sector_seed: 0,
            surface: PlanetSurface::new(),
            locator,
        };
        this.surface.generate(gen, this.radius, this.locator);
        this
    }
}
