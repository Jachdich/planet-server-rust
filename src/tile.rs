use serde::{Deserialize, Serialize, Serializer};
#[derive(Deserialize, Clone, Copy)]
pub enum TileType {
    Air,
    Grass,
    Bush,
    Tree,
    Pine,
    Water,
    Rock,
    House,
    Pineforest,
    Forest,
    Tonk,
    Farm,
    Greenhouse,
    Waterpump,
    Mine,
    Blastfurnace,
    Warehouse,
    Forestry,
    Capsule,
    Road,
    Pipe,
    Cable,
    Powerstation,
    SolarPanel,
    Turbine,
    Lab,
}

impl Serialize for TileType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(*self as u32)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Tile {
    pub ty: TileType,
    pub z: u32,
}
