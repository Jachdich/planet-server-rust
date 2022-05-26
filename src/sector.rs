use crate::star::Star;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Sector {
    x: i32,
    y: i32,
    #[serde(rename = "numStars")]
    num_stars: u32,
    #[serde(skip)]
    generated: bool,
    stars: Vec<Star>,
}

impl Sector {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x, y,
            num_stars: 0,
            generated: false,
            stars: Vec::new()
        }
    }

    pub fn generate(&mut self) {
        self.generated = true;
    }
}
