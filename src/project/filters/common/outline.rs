use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use super::super::super::common::Point;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum Outline {
    Filled(FilledOutline),
    Empty(EmptyOutline),
}

// deriving Deserialize caused failure to match any variant
impl<'de> Deserialize<'de> for Outline {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        pub struct S {
            pub point: Option<[Point; 5]>,
        }

        let s = S::deserialize(deserializer)?;
        match s.point {
            Some(point) => Ok(Outline::Filled(FilledOutline { point })),
            None => Ok(Outline::Empty(EmptyOutline {})),
        }
    }
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct FilledOutline {
    #[validate(custom = "validate_outline")]
    pub point: [Point; 5],
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct EmptyOutline {}

impl Validate for Outline {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            Outline::Filled(outline) => outline.validate(),
            Outline::Empty(_) => Ok(()),
        }
    }
}

fn validate_outline(points: &[Point; 5]) -> Result<(), ValidationError> {
    (points.first().unwrap() == points.last().unwrap())
        .then(|| ())
        .ok_or_else(|| ValidationError::new("invalid outline: first & last points do not match"))
}
