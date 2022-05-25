trait Tile {
    
    fn tick(&mut self, ticks: u64) {

    }

    fn get_error(&self) -> String {
        
    }
}

#
#[derive(Tile)]
struct FarmTile;

impl Tile for FarmTile {
    fn tick(&mut self, ticks: u64) {
        if ticks % 128 == 0 {
            println!("making food");
        }
    }
}
