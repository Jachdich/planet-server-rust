trait Tile {
    
    fn tick(&mut self, ticks: u64) {

    }

    fn get_error(&self) -> String {
        
    }
    fn on_place(&mut self, ticks: u64/*, parent: &PlanetSurface*/) {
        
    }
}

struct FarmTile {
    pub z: u32,
    pub edge: bool
}

impl Tile for FarmTile {
    fn tick(&mut self, ticks: u64) {
        if ticks % 128 == 0 {
            println!("making food");
        }
    }
}
