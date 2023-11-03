use serde::{Deserialize, Serialize};
use serde_with::{serde_as, BoolFromInt};
use validator::Validate;

#[serde_as]
#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct PictureShapeOptions {
    #[serde(rename(
        serialize = "@higherSearchSensitivity",
        deserialize = "higherSearchSensitivity"
    ))]
    #[serde_as(as = "BoolFromInt")]
    pub higher_search_sensitivity: bool,
    #[serde(rename(serialize = "@pictureShape", deserialize = "pictureShape"))]
    pub picture_shape: PictureShape,
    #[serde(rename(serialize = "@sensitivity"))]
    #[validate(range(min = 0, max = 100))]
    pub sensitivity: u8,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum PictureShape {
    Off,
    Free,
    Rectangular,
}
