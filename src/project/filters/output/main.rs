use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    common::{FillZones, ProcessingParams, Zones},
    output_params::OutputParams,
    params::Params,
};

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct Output {
    #[validate]
    pub page: Vec<Page>,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct Page {
    #[serde(rename(serialize = "@id"))]
    pub id: u32,
    pub zones: Zones,
    #[serde(rename = "fill-zones")]
    pub fill_zones: FillZones,
    #[validate]
    pub params: Params,
    #[serde(rename = "processing-params")]
    pub processing_params: ProcessingParams,
    #[serde(rename = "output-params")]
    #[validate]
    pub output_params: OutputParams,
}
