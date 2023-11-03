use serde::{Deserialize, Serialize};
use serde_with::{serde_as, BoolFromInt};
use validator::Validate;

use crate::project::common::FloatSize;

use super::common::{PositiveRegionRect, Rect};

#[serde_as]
#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct PageLayout {
    #[serde(rename(serialize = "@showMiddleRect", deserialize = "showMiddleRect"))]
    #[serde_as(as = "BoolFromInt")]
    pub show_middle_rect: bool,
    #[validate]
    pub page: Vec<Page>,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct Page {
    #[serde(rename(serialize = "@id"))]
    pub id: u32,
    #[validate]
    pub params: Params,
}

#[serde_as]
#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct Params {
    #[serde(rename(serialize = "@autoMargins", deserialize = "autoMargins"))]
    #[serde_as(as = "BoolFromInt")]
    pub auto_margins: bool,
    #[serde(rename = "hardMarginsMM")]
    pub hard_margins_mm: HardMarginsMm,
    #[serde(rename = "pageRect")]
    #[validate]
    pub page_rect: Rect,
    #[serde(rename = "contentRect")]
    #[validate]
    pub content_rect: PositiveRegionRect,
    #[serde(rename = "contentSizeMM")]
    pub content_size: FloatSize,
    pub alignment: Alignment,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct HardMarginsMm {
    #[serde(rename(serialize = "@bottom"))]
    pub bottom: f64,
    #[serde(rename(serialize = "@left"))]
    pub left: f64,
    #[serde(rename(serialize = "@right"))]
    pub right: f64,
    #[serde(rename(serialize = "@top"))]
    pub top: f64,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Alignment {
    #[serde(rename(serialize = "@hor", deserialize = "hor"))]
    pub horizontal: HorizontalAlignment,
    #[serde(rename(serialize = "@null"))]
    #[serde_as(as = "BoolFromInt")]
    pub null: bool, // decides whether alignment is enabled or not
    #[serde(rename(serialize = "@vert", deserialize = "vert"))]
    pub vertical: VerticalAlignment,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}
