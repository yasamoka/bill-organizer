use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct Directories {
    pub directory: Vec<Directory>,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct Directory {
    #[serde(rename(serialize = "@id"))]
    pub id: i32,
    #[serde(rename(serialize = "@path"))]
    pub path: PathBuf,
}
