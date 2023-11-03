use serde::{Deserialize, Serialize};
use validator::Validate;

use super::common::{ImageSettings, Outline, Rotation};

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct Deskew {
    #[validate]
    pub page: Vec<Page>,
    #[serde(rename = "image-settings")]
    pub image_settings: ImageSettings,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct Page {
    #[serde(rename(serialize = "@id"))]
    pub id: u32,
    #[validate]
    pub params: Params,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct Params {
    #[serde(rename(serialize = "@angle"))]
    #[validate(range(min = -45, max = 45))]
    pub angle: f64,
    #[serde(rename(serialize = "@mode"))]
    pub mode: Mode,
    #[validate]
    pub dependencies: Dependencies,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum Mode {
    Auto,
    Manual,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct Dependencies {
    pub rotation: Rotation,
    #[serde(rename = "page-outline")]
    #[validate]
    pub page_outline: Outline,
}
