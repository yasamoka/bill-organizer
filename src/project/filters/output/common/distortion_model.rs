use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as};

use super::super::super::super::common::Point;

// ScanTailor seems to leave in a distortion-model
// if dewarping was enabled at some point, even if
// it is now off, without cleaning up, even leaving
// in artifacts from different modes
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct DistortionModel {
    #[serde(rename = "top-curve")]
    pub top_curve: Curve,
    #[serde(rename = "bottom-curve")]
    pub bottom_curve: Curve,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Curve {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xspline: Option<XSpline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<Base64>")]
    pub polyline: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct XSpline {
    point: Vec<Point>,
}
