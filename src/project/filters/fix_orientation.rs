use serde::{Deserialize, Serialize};

use super::common::ImageSettings;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct FixOrientation {
    pub image: Vec<Image>,
    #[serde(rename = "image-settings")]
    pub image_settings: ImageSettings,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Image {
    #[serde(rename(serialize = "@id"))]
    pub id: u32,
    pub rotation: Rotation,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Rotation {
    #[serde(rename(serialize = "@degrees"))]
    pub degrees: Degrees,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Degrees {
    #[serde(rename = "90")]
    _90,
    #[serde(rename = "180")]
    _180,
    #[serde(rename = "270")]
    _270,
}
