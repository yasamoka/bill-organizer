use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Files {
    pub file: Vec<File>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct File {
    #[serde(rename(serialize = "@dirId", deserialize = "dirId"))]
    pub dir_id: u32,

    #[serde(rename(serialize = "@id"))]
    pub id: u32,

    #[serde(rename(serialize = "@name"))]
    pub name: String,
}
