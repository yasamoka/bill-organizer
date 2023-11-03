use serde::{Deserialize, Serialize};
use serde_with::{
    chrono::{DateTime, Utc},
    serde_as, BoolFromInt, TimestampSeconds,
};
use validator::Validate;

use crate::project::filters::common::PositiveRegionRect;

use super::{
    super::super::common::{IntSize, DPI},
    super::common::Outline,
    common::{
        ColorParams, DewarpingOptions, DistortionModel, FillZones, PictureShapeOptions,
        ProcessingParams, Splitting, Zones,
    },
};

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct OutputParams {
    #[validate]
    pub image: Image,
    pub source_file: File,
    pub file: File,
    // TODO: validate both in struct to show up only when split output is enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreground_file: Option<File>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_file: Option<File>,
    // TODO: validate in struct to show up only when color mode is chosen
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automask: Option<File>,
    // TODO: validate in struct to show up when split output BW is enabled or ...?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speckles: Option<File>,
    pub zones: Zones,
    #[serde(rename = "fill-zones")]
    pub fill_zones: FillZones,
}

#[serde_as]
#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct Image {
    #[serde(rename(serialize = "@blackOnWhite", deserialize = "blackOnWhite"))]
    #[serde_as(as = "BoolFromInt")]
    pub black_on_white: bool,
    // no idea what this is
    #[serde(rename(serialize = "@depthPerception", deserialize = "depthPerception"))]
    pub depth_perception: u32,
    #[serde(rename(serialize = "@despeckleLevel", deserialize = "despeckleLevel"))]
    #[validate(range(min = 1, max = 3))]
    pub despeckle_level: f64,
    pub size: IntSize,
    #[serde(rename = "content-rect")]
    #[validate]
    pub content_rect: PositiveRegionRect,
    #[serde(rename = "crop-area")]
    #[validate]
    pub crop_area: Outline,
    #[serde(rename = "partial-xform")]
    pub partial_transform: PartialTransform,
    pub dpi: DPI,
    #[serde(rename = "color-params")]
    #[validate]
    pub color_params: ColorParams,
    pub splitting: Splitting,
    #[serde(rename = "picture-shape-options")]
    #[validate]
    pub picture_shape_options: PictureShapeOptions,
    // avoided using serde(flatten) due to https://github.com/tafia/quick-xml/issues/286#issuecomment-1133762830
    #[serde(rename = "distortion-model", skip_serializing_if = "Option::is_none")]
    pub distortion_model: Option<DistortionModel>,
    #[serde(rename = "dewarping-options")]
    #[validate]
    pub dewarping_options: DewarpingOptions,
    #[serde(rename = "processing-params")]
    pub processing_params: ProcessingParams,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PartialTransform {
    pub m11: f64,
    pub m12: f64,
    pub m21: f64,
    pub m22: f64,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct File {
    #[serde(rename(serialize = "@mtime"))]
    #[serde_as(as = "TimestampSeconds<i64>")]
    pub mtime: DateTime<Utc>,
    #[serde(rename(serialize = "@size"))]
    pub size: u64,
}
