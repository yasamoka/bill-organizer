use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Pages {
    pub page: Vec<Page>,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Page {
    #[serde(rename(serialize = "@id"))]
    pub id: u32,

    #[serde(rename(serialize = "@imageId", deserialize = "imageId"))]
    pub image_id: u32,

    #[serde(
        rename(serialize = "@selected"),
        skip_serializing_if = "Option::is_none"
    )]
    pub selected: Option<Selected>,

    #[serde(rename(serialize = "@subPage", deserialize = "subPage"))]
    pub sub_page: SubPage,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SubPage {
    Single,
    Left,
    Right,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Selected {
    #[serde(rename = "selected")]
    Selected,
}
