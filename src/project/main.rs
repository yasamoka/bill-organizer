use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use validator::Validate;

use super::{
    directories::Directories, file_name_disambiguation::FileNameDisambiguation, files::Files,
    filters::Filters, images::Images, layout_direction::LayoutDirection, pages::Pages,
};

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(rename = "project")]
pub struct ProjectXML {
    #[serde(rename(serialize = "@layoutDirection", deserialize = "layoutDirection"))]
    pub layout_direction: LayoutDirection,
    #[serde(rename(serialize = "@outputDirectory", deserialize = "outputDirectory"))]
    pub output_dir: PathBuf,
    #[serde(rename(serialize = "@version"))]
    #[validate(range(min = 3, max = 3))]
    pub version: u8,
    #[validate]
    pub directories: Directories,
    pub files: Files,
    pub images: Images,
    pub pages: Pages,
    #[serde(rename = "file-name-disambiguation")]
    pub file_name_disambiguation: FileNameDisambiguation,
    #[validate]
    pub filters: Filters,
}
