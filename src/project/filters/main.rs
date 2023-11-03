use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    deskew::Deskew, fix_orientation::FixOrientation, output::Output, page_layout::PageLayout,
    page_split::PageSplit, select_content::SelectContent,
};

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct Filters {
    #[serde(rename = "fix-orientation")]
    pub fix_orientation: FixOrientation,
    #[serde(rename = "page-split")]
    #[validate]
    pub page_split: PageSplit,
    #[validate]
    pub deskew: Deskew,
    #[serde(rename = "select-content")]
    #[validate]
    pub select_content: SelectContent,
    #[serde(rename = "page-layout")]
    #[validate]
    pub page_layout: PageLayout,
    #[validate]
    pub output: Output,
}
