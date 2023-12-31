use serde::{Deserialize, Serialize};
use serde_with::{serde_as, BoolFromInt};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ImageSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<Vec<ImageSettingsPage>>,
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
