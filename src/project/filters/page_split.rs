use serde::{de, Deserialize, Serialize};
use validator::Validate;

use crate::project::common::{IntSize, Point};

use super::common::{Outline, Rotation};

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct PageSplit {
    #[serde(rename(serialize = "@defaultLayoutType", deserialize = "defaultLayoutType"))]
    pub default_layout_type: DefaultLayoutType,
    #[validate]
    pub image: Vec<Image>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum DefaultLayoutType {
    AutoDetect,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct Image {
    #[serde(rename(serialize = "@id"))]
    pub id: u32,
    #[serde(rename(serialize = "@layoutType", deserialize = "layoutType"))]
    pub layout_type: LayoutType,
    #[validate]
    pub params: Params,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct Params {
    #[serde(rename(serialize = "@mode", deserialize = "mode"))]
    pub mode: Mode,
    #[validate]
    pub pages: Pages,
    pub dependencies: Dependencies,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum Mode {
    Auto,
    Manual,
}

#[derive(Serialize, Debug)]
#[serde(tag = "@type")]
#[serde(rename_all(serialize = "kebab-case"))]
pub enum Pages {
    SingleUncut(SingleUncutPages),
    SingleCut(SingleCutPages),
    TwoPages(TwoPagesPages),
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct SingleUncutPages {
    #[validate]
    pub outline: Outline,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct SingleCutPages {
    #[validate]
    pub outline: Outline,
    pub cutter1: Cutter,
    pub cutter2: Cutter,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct TwoPagesPages {
    #[validate]
    pub outline: Outline,
    pub cutter1: Cutter,
}

impl<'de> Deserialize<'de> for Pages {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct S {
            #[serde(rename(serialize = "@type", deserialize = "type"))]
            pub r#type: LayoutType,
            pub outline: Outline,
            pub cutter1: Option<Cutter>,
            pub cutter2: Option<Cutter>,
        }

        let S {
            r#type,
            outline,
            cutter1,
            cutter2,
        } = S::deserialize(deserializer)?;

        match (r#type, cutter1, cutter2) {
            (LayoutType::SingleUncut, None, None) => {
                Ok(Pages::SingleUncut(SingleUncutPages { outline }))
            }
            (LayoutType::SingleCut, Some(cutter1), Some(cutter2)) => {
                Ok(Pages::SingleCut(SingleCutPages {
                    outline,
                    cutter1,
                    cutter2,
                }))
            }
            (LayoutType::TwoPages, Some(cutter1), None) => {
                Ok(Pages::TwoPages(TwoPagesPages { outline, cutter1 }))
            }
            _ => Err(de::Error::custom("unknown pages structure")),
        }
    }
}

impl Validate for Pages {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            Pages::SingleUncut(pages) => pages.validate(),
            Pages::SingleCut(pages) => pages.validate(),
            Pages::TwoPages(pages) => pages.validate(),
        }
    }
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct Cutter {
    p1: Point,
    p2: Point,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Dependencies {
    pub rotation: Rotation,
    pub size: IntSize,
    #[serde(rename = "layoutType")]
    pub layout_type: LayoutType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum LayoutType {
    SingleUncut,
    SingleCut,
    TwoPages,
}
