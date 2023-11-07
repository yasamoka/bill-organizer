use serde::{Deserialize, Serialize};

use super::common::{IntSize, DPI};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Images {
    pub image: Vec<Image>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Image {
    #[serde(rename(serialize = "@fileId", deserialize = "fileId"))]
    pub file_id: u32,

    #[serde(rename(serialize = "@fileImage", deserialize = "fileImage"))]
    pub file_image: u32,

    #[serde(rename(serialize = "@id"))]
    pub id: u32,

    #[serde(rename(serialize = "@subPages", deserialize = "subPages"))]
    pub sub_pages: u32,

    pub size: IntSize,

    pub dpi: DPI,
}
