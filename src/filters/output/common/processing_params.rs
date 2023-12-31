use serde::{Deserialize, Serialize};
use serde_with::{serde_as, BoolFromInt};

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ProcessingParams {
    #[serde(rename(serialize = "@autoZonesFound", deserialize = "autoZonesFound"))]
    pub auto_zones_found: u32,

    #[serde(rename(
        serialize = "@blackOnWhiteSetManually",
        deserialize = "blackOnWhiteSetManually"
    ))]
    #[serde_as(as = "BoolFromInt")]
    pub black_on_white_set_manually: bool,
}
