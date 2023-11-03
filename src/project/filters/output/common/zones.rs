use serde::{Deserialize, Serialize};

use super::super::super::common::Outline;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Zones {
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<Vec<Zone>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Zone {
    pub spline: Outline,
    pub properties: Properties,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Properties {
    property: Vec<Property>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum Property {
    #[serde(rename = "PictureZoneProperty")]
    PictureZone(PictureZoneProperty),
    #[serde(rename = "ZoneCategoryProperty")]
    ZoneCategory(ZoneCategoryProperty),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PictureZoneProperty {
    layer: Layer,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Layer {
    Painter2,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ZoneCategoryProperty {
    #[serde(rename(serialize = "@zoneCategory", deserialize = "zoneCategory"))]
    category: ZoneCategory,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ZoneCategory {
    Auto,
}
