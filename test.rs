#[derive(Debug)]
struct Resources {
    a: u32,
}

#[derive(Debug, Clone)]
struct Tile {
    z: u32,
}

#[derive(Debug)]
struct PlanetSurface {
    t: Vec<Tile>,
    r: Resources
}

impl Tile {
    fn tick(&mut self, res: &mut Resources, tiles: Vec<Tile>) {
        res.a += self.z;
    }
}

impl PlanetSurface {
    fn tick(&mut self) {
        let res: &mut Resources = &mut self.r;
        let len = self.t.len();
        let tiles = self.t.as_mut_slice();
        for i in 0..len {
            let (before, midafter) = tiles.split_at_mut(i);
            let others;
            let tile;
            if i < len - 1 {
                let (mid, after) = midafter.split_at_mut(i+1);
                tile = &mut mid[0];
                others = [before, &mut [tile.clone()], after].concat();
            } else {
                tile = &mut midafter[0];
                others = [before, &mut [tile.clone()]].concat();
            }
            tile.tick(res, others);
        }
    }
}

fn main() {
    let mut ps = PlanetSurface {
        t: vec![Tile { z: 1 }, Tile { z: 2 }, Tile { z: 3 }],
        r: Resources { a: 69 }
    };
    ps.tick();
    println!("{:?}", ps);
}
