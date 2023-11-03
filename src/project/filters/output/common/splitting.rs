use serde::{Deserialize, Serialize};
use serde_with::{serde_as, BoolFromInt};

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Splitting {
    #[serde(rename(serialize = "@originalBackground", deserialize = "originalBackground"))]
    #[serde_as(as = "BoolFromInt")]
    pub original_background: bool,
    #[serde(rename(serialize = "@splitOutput", deserialize = "splitOutput"))]
    #[serde_as(as = "BoolFromInt")]
    pub split_output: bool,
    #[serde(rename(serialize = "@splittingMode", deserialize = "splittingMode"))]
    pub splitting_mode: SplittingMode,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SplittingMode {
    BW,
    Color,
}
