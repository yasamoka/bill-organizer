use serde::{Deserialize, Serialize};
use serde_with::{serde_as, BoolFromInt};
use validator::Validate;

use super::{
    super::super::{common::DPI, filters::output::common::Splitting},
    common::{ColorParams, DewarpingOptions, DistortionModel, PictureShapeOptions},
};

#[serde_as]
#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct Params {
    #[serde(rename(serialize = "@blackOnWhite", deserialize = "blackOnWhite"))]
    #[serde_as(as = "BoolFromInt")]
    pub black_on_white: bool,
    // no idea what this is
    #[serde(rename(serialize = "@depthPerception", deserialize = "depthPerception"))]
    pub depth_perception: u32,
    #[serde(rename(serialize = "@despeckleLevel", deserialize = "despeckleLevel"))]
    #[validate(range(min = 1, max = 3))]
    pub despeckle_level: f64,
    #[serde(rename = "distortion-model", skip_serializing_if = "Option::is_none")]
    pub distortion_model: Option<DistortionModel>,
    #[serde(rename = "picture-shape-options")]
    #[validate]
    pub picture_shape_options: PictureShapeOptions,
    #[serde(rename = "dewarping-options")]
    #[validate]
    pub dewarping_options: DewarpingOptions,
    pub dpi: DPI,
    #[serde(rename = "color-params")]
    pub color_params: ColorParams,
    pub splitting: Splitting,
}
