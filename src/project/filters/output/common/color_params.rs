use serde::{Deserialize, Serialize};
use serde_with::{serde_as, BoolFromInt};
use validator::Validate;

// TODO: change to enum depending on color mode
#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct ColorParams {
    #[serde(rename(serialize = "@colorMode", deserialize = "colorMode"))]
    pub color_mode: ColorMode,
    #[serde(rename = "color-or-grayscale")]
    #[validate]
    pub color_or_grayscale: Option<ColorOrGrayscale>, // TODO: is this actually an option?
    #[validate]
    pub bw: Option<BW>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ColorMode {
    #[serde(rename = "bw")]
    BlackAndWhite,
    #[serde(rename = "colorOrGray")]
    ColorOrGrayscale,
    #[serde(rename = "mixed")]
    Mixed,
}

#[serde_as]
#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct ColorOrGrayscale {
    #[serde(rename(serialize = "@fillMargins", deserialize = "fillMargins"))]
    #[serde_as(as = "BoolFromInt")]
    pub fill_margins: bool,
    #[serde(rename(serialize = "@fillOffcut", deserialize = "fillOffcut"))]
    #[serde_as(as = "BoolFromInt")]
    pub fill_offcut: bool,
    #[serde(rename(serialize = "@fillingColor", deserialize = "fillingColor"))]
    pub filling_color: FillingColor,
    #[serde(rename(
        serialize = "@normalizeIlluminationColor",
        deserialize = "normalizeIlluminationColor"
    ))]
    #[serde_as(as = "BoolFromInt")]
    pub normalize_illumination_color: bool,
    #[serde(rename = "posterization-options")]
    #[validate]
    pub posterization_options: PosterizationOptions,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum FillingColor {
    Background,
    White,
    Black,
}

#[serde_as]
#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct PosterizationOptions {
    #[serde(rename(serialize = "@enabled"))]
    #[serde_as(as = "BoolFromInt")]
    pub enabled: bool,
    #[serde(rename(serialize = "@forceBlackAndWhite", deserialize = "forceBlackAndWhite"))]
    #[serde_as(as = "BoolFromInt")]
    pub force_black_and_white: bool,
    #[serde(rename(serialize = "@level"))]
    #[validate(range(min = 2))]
    pub level: u8,
    #[serde(rename(
        serialize = "@normalizationEnabled",
        deserialize = "normalizationEnabled"
    ))]
    #[serde_as(as = "BoolFromInt")]
    pub normalization_enabled: bool,
}

#[serde_as]
#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct BW {
    #[serde(rename(serialize = "@binarizationMethod", deserialize = "binarizationMethod"))]
    pub binarization_method: BinarizationMethod,
    #[serde(rename(
        serialize = "@morphologicalSmoothing",
        deserialize = "morphologicalSmoothing"
    ))]
    #[serde_as(as = "BoolFromInt")]
    pub morphological_smoothing: bool,
    #[serde(rename(
        serialize = "@normalizeIlluminationBW",
        deserialize = "normalizeIlluminationBW"
    ))]
    #[serde_as(as = "BoolFromInt")]
    pub normalize_illumination_bw: bool,
    #[serde(rename(serialize = "@sauvolaCoef", deserialize = "sauvolaCoef"))]
    #[validate(range(min = 0.01, max = 9.99))]
    pub sauvola_coeff: f64,
    #[serde(rename(
        serialize = "@savitzkyGolaySmoothing",
        deserialize = "savitzkyGolaySmoothing"
    ))]
    #[serde_as(as = "BoolFromInt")]
    pub savitzky_golay_smoothing: bool,
    #[serde(rename(serialize = "@thresholdAdj", deserialize = "thresholdAdj"))]
    #[validate(range(min = -100, max = 100))]
    pub threshold_adj: i8,
    #[serde(rename(serialize = "@windowSize", deserialize = "windowSize"))]
    #[validate(range(min = 3, max = 9999))]
    pub window_size: u16,
    #[serde(rename(serialize = "@wolfCoef", deserialize = "wolfCoef"))]
    #[validate(range(min = 0.01, max = 9.99))]
    pub wolf_coeff: f64,
    #[serde(rename(serialize = "@wolfLowerBound", deserialize = "wolfLowerBound"))]
    #[validate(range(min = 1, max = 254))]
    pub wolf_lower_bound: u8,
    #[serde(rename(serialize = "@wolfUpperBound", deserialize = "wolfUpperBound"))]
    #[validate(range(min = 1, max = 254))]
    pub wolf_upper_bound: u8,
    #[serde(rename = "color-segmenter-options")]
    #[validate]
    pub color_segmenter_options: ColorSegmenterOptions,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum BinarizationMethod {
    Otsu,
    Sauvola,
    Wolf,
    EdgePlus,
    BlurDiv,
    EdgeDiv,
}

#[serde_as]
#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct ColorSegmenterOptions {
    #[serde(rename(
        serialize = "@blueThresholdAdjustment",
        deserialize = "blueThresholdAdjustment"
    ))]
    #[validate(range(min = -99, max = 99))]
    pub blue_threshold_adjustment: i8,
    #[serde(rename(serialize = "@enabled"))]
    #[serde_as(as = "BoolFromInt")]
    pub enabled: bool,
    #[serde(rename(
        serialize = "@greenThresholdAdjustment",
        deserialize = "greenThresholdAdjustment"
    ))]
    #[validate(range(min = -99, max = 99))]
    pub green_threshold_adjustment: i8,
    #[serde(rename(serialize = "@noiseReduction", deserialize = "noiseReduction"))]
    #[validate(range(min = 0, max = 999))]
    pub noise_reduction: u16,
    #[serde(rename(
        serialize = "@redThresholdAdjustment",
        deserialize = "redThresholdAdjustment"
    ))]
    #[validate(range(min = -99, max = 99))]
    pub red_threshold_adjustment: i8,
}
