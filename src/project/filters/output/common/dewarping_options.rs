use serde::{Deserialize, Serialize};
use serde_with::{serde_as, BoolFromInt};
use validator::Validate;

#[serde_as]
#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct DewarpingOptions {
    #[serde(rename(serialize = "@mode"))]
    pub mode: DewarpingMode,
    #[serde(rename(serialize = "@postDeskew", deserialize = "postDeskew"))]
    #[serde_as(as = "BoolFromInt")]
    pub post_deskew: bool,
    #[serde(rename(serialize = "@postDeskewAngle", deserialize = "postDeskewAngle"))]
    // unsure if this is the correct range
    #[validate(range(min = -45, max = 45))]
    pub post_deskew_angle: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum DewarpingMode {
    Off,
    Auto,
    Marginal,
}
