use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Size<T> {
    #[serde(rename(serialize = "@height"))]
    pub height: T,

    #[serde(rename(serialize = "@width"))]
    pub width: T,
}

pub type IntSize = Size<u32>;
pub type FloatSize = Size<f64>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct DPI {
    #[serde(rename(serialize = "@horizontal"))]
    pub horizontal: f64,

    #[serde(rename(serialize = "@vertical"))]
    pub vertical: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Point {
    #[serde(rename(serialize = "@x"))]
    pub x: f64,

    #[serde(rename(serialize = "@y"))]
    pub y: f64,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}
