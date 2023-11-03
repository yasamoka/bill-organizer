use serde::{Deserialize, Serialize};
use serde_with::{serde_as, BoolFromInt};
use validator::{Validate, ValidationError};

use crate::project::common::Point;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ImageSettings {
    pub page: Vec<ImageSettingsPage>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ImageSettingsPage {
    #[serde(rename(serialize = "@id"))]
    pub id: u32,
    #[serde(rename = "image-params")]
    pub image_params: ImageParams,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ImageParams {
    #[serde(rename(serialize = "@blackOnWhite", deserialize = "blackOnWhite"))]
    #[serde_as(as = "BoolFromInt")]
    pub black_on_white: bool,
    #[serde(rename(serialize = "@bwThreshold", deserialize = "bwThreshold"))]
    pub bw_threshold: u8,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Rotation {
    #[serde(rename(serialize = "@degrees"))]
    pub degrees: Degrees,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Degrees {
    #[serde(rename = "0")]
    _0,
    #[serde(rename = "90")]
    _90,
    #[serde(rename = "180")]
    _180,
    #[serde(rename = "270")]
    _270,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct Outline {
    #[validate(custom = "validate_outline")]
    pub point: [Point; 5],
}

fn validate_outline(points: &[Point; 5]) -> Result<(), ValidationError> {
    (points.first().unwrap() == points.last().unwrap())
        .then(|| ())
        .ok_or_else(|| ValidationError::new("invalid outline: first & last points do not match"))
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct Rect {
    #[serde(rename(serialize = "@height"))]
    #[validate(range(min = 0))]
    pub height: f64,
    #[serde(rename(serialize = "@width"))]
    #[validate(range(min = 0))]
    pub width: f64,
    #[serde(rename(serialize = "@x"))]
    pub x: f64,
    #[serde(rename(serialize = "@y"))]
    pub y: f64,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct PositiveRegionRect {
    #[serde(rename(serialize = "@height"))]
    #[validate(range(min = 0))]
    pub height: f64,
    #[serde(rename(serialize = "@width"))]
    #[validate(range(min = 0))]
    pub width: f64,
    #[serde(rename(serialize = "@x"))]
    #[validate(range(min = 0))]
    pub x: f64,
    #[serde(rename(serialize = "@y"))]
    #[validate(range(min = 0))]
    pub y: f64,
}
