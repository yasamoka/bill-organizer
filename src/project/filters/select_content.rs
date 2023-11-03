use serde::{Deserialize, Serialize};
use serde_with::{serde_as, BoolFromInt};
use validator::Validate;

use super::{
    super::common::FloatSize,
    common::{Outline, PositiveRegionRect, Rect},
};

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct SelectContent {
    #[serde(rename(
        serialize = "@pageDetectionTolerance",
        deserialize = "pageDetectionTolerance"
    ))]
    pub page_detection_tolerance: f64,
    #[validate]
    pub page: Vec<Page>,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct Page {
    #[serde(rename(serialize = "@id"))]
    pub id: u32,
    #[validate]
    pub params: PageParams,
}

#[serde_as]
#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct PageParams {
    #[serde(rename(
        serialize = "@contentDetectionMode",
        deserialize = "contentDetectionMode"
    ))]
    pub content_detection_mode: ContentDetectionMode,
    #[serde(rename(serialize = "@fineTuneCorners", deserialize = "fineTuneCorners"))]
    #[serde_as(as = "BoolFromInt")]
    pub fine_tune_corners: bool,
    #[serde(rename(serialize = "@pageDetectionMode", deserialize = "pageDetectionMode"))]
    pub page_detection_mode: PageDetectionMode,
    #[serde(rename = "content-rect")]
    #[validate]
    pub content_rect: PositiveRegionRect,
    #[serde(rename = "page-rect")]
    #[validate]
    pub page_rect: Rect,
    #[serde(rename = "content-size-mm")]
    pub content_size_mm: FloatSize,
    #[validate]
    pub dependencies: Dependencies,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum PageDetectionMode {
    Disabled,
    Auto,
    Manual,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum ContentDetectionMode {
    Disabled,
    Auto,
    Manual,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct Dependencies {
    #[serde(rename = "rotated-page-outline")]
    #[validate]
    pub rotated_page_outline: Outline,
    pub params: DependenciesParams,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct DependenciesParams {
    #[serde(rename(
        serialize = "@contentDetectionMode",
        deserialize = "contentDetectionMode"
    ))]
    pub content_detection_mode: ContentDetectionMode,
    #[serde(rename(serialize = "@fineTuneCorners", deserialize = "fineTuneCorners"))]
    #[serde_as(as = "BoolFromInt")]
    pub fine_tune_corners: bool,
    #[serde(rename(serialize = "@pageDetectionMode", deserialize = "pageDetectionMode"))]
    pub page_detection_mode: PageDetectionMode,
}
