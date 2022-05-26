use std::collections::HashMap;
use crate::sector::Sector;

pub struct SectorMap {
    sectors: HashMap<u64, Sector>,    
}

fn xy_to_index(x: i32, y: i32) -> u64 {
    //rust trickery to smoosh the bits of 2 signed integers
    ((x as u32) as u64) << 32 | (y as u32) as u64
}

impl SectorMap {
    pub fn new() -> Self {
        Self {
            sectors: HashMap::new(),
        }
    }

    pub fn get_sector_at(&mut self, x: i32, y: i32) -> &Sector {
        let idx = xy_to_index(x, y);
        if !self.sectors.contains_key(&idx) {
            let mut sector = Sector::new(x, y);
            sector.generate();
            self.sectors.insert(idx, sector);
        }
        &self.sectors.get(&idx).unwrap()
    }
}