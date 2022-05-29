use std::ops::RangeInclusive;
use serde::Serialize;
use serde::Serializer;
use serde::Deserialize;
use serde::Deserializer;
use serde::ser::SerializeSeq;
use std::marker::PhantomData;
use serde::de::Visitor;
use serde::de::SeqAccess;
use serde::de::Error;
use rand::Rng;

pub struct Range<T>(RangeInclusive<T>);

impl<T: std::cmp::PartialOrd + rand::distributions::uniform::SampleUniform + Clone> Range<T> {
    pub fn gen_rand(&self) -> T {
        rand::thread_rng().gen_range(self.0.clone())
    }
}

impl<T> Serialize for Range<T> where T: Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
        where S: Serializer {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(self.0.start())?;
        seq.serialize_element(self.0.end())?;
        seq.end()
    }
}

struct ArrayVisitor<A> {
    marker: PhantomData<A>,
}

impl<A> ArrayVisitor<A> {
    fn new() -> Self {
        ArrayVisitor {
            marker: PhantomData,
        }
    }
}
impl<'de, T> Visitor<'de> for ArrayVisitor<[T; 2]>
where
    T: Deserialize<'de> + Copy,
{
    type Value = [T; 2];

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str(concat!("an array of length ", 2))
    }

    #[inline]
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        Ok([
            match seq.next_element()? {
                Some(val) => val,
                None => return Err(Error::invalid_length(0, &self)),
            },
            match seq.next_element()? {
                Some(val) => val,
                None => return Err(Error::invalid_length(1, &self)),
            },
        ])
    }
}


impl<'a, T> Deserialize<'a> for Range<T> where T: Deserialize<'a> + Copy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> 
        where D: Deserializer<'a> {
        let arr = deserializer.deserialize_tuple(2, ArrayVisitor::<[T; 2]>::new())?;
        Ok(Range{0:arr[0]..=arr[1]})
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct StarGen {
    pub num_planets: Range<u32>,
    pub rad: Range<u32>,
    pub temp: Range<u32>,
    pub noise_scl: Range<f64>,
    pub noise_effect: Range<f64>
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PlanetGen {
    pub rad: Range<u32>,
    pub gen_chance: Range<f64>,
    pub gen_noise: Range<f64>,
    pub gen_noise_plants: Range<f64>,
    pub base_colour: Range<u8>,
    pub angular_velocity_mult: f64,
    pub num_colours: Range<u32>,
    pub sea_level: Range<i32>
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SectorGen {
    pub num_stars: Range<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GenParams {
    pub planet: PlanetGen,
    pub sector: SectorGen,
    pub star: StarGen,
    pub level_seed: i32
}

impl GenParams {
    pub fn load_from(path: &str) -> Result<Self, std::io::Error> {
        let contents = std::fs::read_to_string(path)?;
        let res: Self = serde_json::from_str(&contents)?;
        Ok(res)
    }
}
