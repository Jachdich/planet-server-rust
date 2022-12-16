extern crate tile_derive;
use std::error::Error;
use tile_derive::Tile;

#[derive(Debug)]
enum TileType {
    GrassTile,
    FarmTile,
}

#[derive(Debug)]
struct TileError {
    message: String,
}

impl std::fmt::Display for TileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TileError({})", self.message)
    }
}

impl Error for TileError {
    fn description(&self) -> &str {
        &self.message
    }
}

trait Tile {
    fn get_type(&self) -> TileType;
    fn employ_people(&self, num: i32) -> Result<(), TileError> {
        println!("Employing {} people", num);
        Ok(())
    }
    fn tick(&self) -> Result<(), TileError> {
        Ok(())
    }
}

#[derive(Tile)]
struct GrassTile;

#[derive(Tile)]
struct FarmTile;

impl FarmTile {
    fn tick(&self) -> Result<(), TileError> {
        self.employ_people(2)?;
        Ok(())
    }
}

fn main() {
    let mut a: Vec<Box<dyn Tile>> = Vec::new();
    //a.push(Box::new(GrassTile));

    a.push(Box::new(FarmTile));
    for tile in &a {
        let tile_error = tile.tick();
        println!("{:?} {:?}", tile.get_type(), tile_error);
    }
}
