use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct FileNameDisambiguation {
    pub mapping: Vec<Mapping>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Mapping {
    #[serde(rename(serialize = "@file"))]
    pub file: u32,
    #[serde(rename(serialize = "@label"))]
    pub label: String,
}
