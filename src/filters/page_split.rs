use serde::{de, Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::common::{IntSize, Point};

use super::common::{FilledOutline, Rotation};

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct PageSplit {
    #[serde(rename(serialize = "@defaultLayoutType", deserialize = "defaultLayoutType"))]
    pub default_layout_type: DefaultLayoutType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub image: Option<Vec<Image>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum DefaultLayoutType {
    AutoDetect,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
#[validate(schema(function = "validate_image"))]
pub struct Image {
    #[serde(rename(serialize = "@id"))]
    pub id: u32,

    #[serde(rename(serialize = "@layoutType", deserialize = "layoutType"))]
    pub layout_type: LayoutType,

    #[validate]
    pub params: Params,
}

// TODO: check for version differences causing incoherence and auto-detect
fn validate_image(image: &Image) -> Result<(), ValidationError> {
    match (
        &image.layout_type,
        &image.params.pages,
        &image.params.dependencies.layout_type,
    ) {
        (LayoutType::SingleUncut, Pages::SingleUncut(_), LayoutType::SingleUncut)
        | (LayoutType::SingleCut, Pages::SingleCut(_), LayoutType::SingleCut)
        | (LayoutType::TwoPages, Pages::TwoPages(_), LayoutType::TwoPages) => Ok(()),
        _ => Err(ValidationError::new("layout types do not match")),
    }
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

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
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
    pub outline: FilledOutline,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct SingleCutPages {
    #[validate]
    pub outline: FilledOutline,

    pub cutter1: Cutter,

    pub cutter2: Cutter,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct TwoPagesPages {
    #[validate]
    pub outline: FilledOutline,

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

            pub outline: FilledOutline,

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

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum LayoutType {
    SingleUncut,
    SingleCut,
    TwoPages,
}

#[cfg(test)]
mod test {
    use serde_xml_rs::from_str;
    use validator::Validate;

    use crate::{
        filters::page_split::{Image, LayoutType, Mode, Pages},
        ProjectXML,
    };

    fn get_image<'a>(content: &str, validate: bool) -> Image {
        let project: ProjectXML = from_str(content).unwrap();
        if validate {
            project.validate().unwrap();
        }
        let mut images = project.filters.page_split.image.unwrap();
        assert_eq!(images.len(), 1);
        images.pop().unwrap()
    }

    #[test]
    fn it_deserializes_and_validates_single_uncut_pages() {
        const CONTENT: &str = r#"<project layoutDirection="LTR" outputDirectory="/home/ramzi/src/bill-organizer/out" version="3"><directories><directory id="1" path="/home/ramzi/src/bill-organizer"/></directories><files><file dirId="1" id="2" name="in.jpg"/></files><images><image fileId="2" fileImage="0" id="3" subPages="1"><size height="4961" width="7016"/><dpi horizontal="600" vertical="600"/></image></images><pages><page id="4" imageId="3" selected="selected" subPage="single"/></pages><file-name-disambiguation><mapping file="2" label="0"/></file-name-disambiguation><filters><fix-orientation><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="193"/></page></image-settings></fix-orientation><page-split defaultLayoutType="auto-detect"><image id="3" layoutType="single-uncut"><params mode="auto"><pages type="single-uncut"><outline><point x="0" y="0"/><point x="7016" y="0"/><point x="7016" y="4961"/><point x="0" y="4961"/><point x="0" y="0"/></outline></pages><dependencies><rotation degrees="0"/><size height="4961" width="7016"/><layoutType>single-uncut</layoutType></dependencies></params></image></page-split><deskew><page id="4"><params angle="0" mode="auto"><dependencies><rotation degrees="0"/><page-outline/></dependencies></params></page><image-settings/></deskew><select-content pageDetectionTolerance="0.1"><page id="4"><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="disabled"><content-rect height="0" width="0" x="0" y="0"/><page-rect height="7015.733999999999" width="4960.62" x="0" y="0"/><content-size-mm height="-1" width="-1"/><dependencies><rotated-page-outline/><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="disabled"/></dependencies></params></page></select-content><page-layout showMiddleRect="1"><page id="4"><params autoMargins="0"><hardMarginsMM bottom="5" left="10" right="10" top="5"/><pageRect height="0" width="0" x="0" y="0"/><contentRect height="0" width="0" x="0" y="0"/><contentSizeMM height="-1" width="-1"/><alignment hor="center" null="0" vert="center"/></params></page></page-layout><output><page id="4"><zones/><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="bw"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/></page></output></filters></project>"#;
        let image = get_image(CONTENT, true);

        assert_eq!(image.layout_type, LayoutType::SingleUncut);

        match image.params.pages {
            Pages::SingleUncut(_) => {}
            _ => assert!(false),
        }
    }

    #[test]
    fn it_deserializes_and_validates_single_cut_pages() {
        const CONTENT: &str = r#"<project layoutDirection="LTR" outputDirectory="/home/ramzi/src/bill-organizer/out" version="3"><directories><directory id="1" path="/home/ramzi/src/bill-organizer"/></directories><files><file dirId="1" id="2" name="in.jpg"/></files><images><image fileId="2" fileImage="0" id="3" subPages="1"><size height="4961" width="7016"/><dpi horizontal="600" vertical="600"/></image></images><pages><page id="4" imageId="3" selected="selected" subPage="single"/></pages><file-name-disambiguation><mapping file="2" label="0"/></file-name-disambiguation><filters><fix-orientation><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="193"/></page></image-settings></fix-orientation><page-split defaultLayoutType="auto-detect"><image id="3" layoutType="single-cut"><params mode="auto"><pages type="single-cut"><outline><point x="0" y="0"/><point x="7016" y="0"/><point x="7016" y="4961"/><point x="0" y="4961"/><point x="0" y="0"/></outline><cutter1><p1 x="1361.028490094375" y="0"/><p2 x="1382.679436866696" y="4962"/></cutter1><cutter2><p1 x="2817.82175895945" y="4962"/><p2 x="2861.124476937195" y="0"/></cutter2></pages><dependencies><rotation degrees="0"/><size height="4961" width="7016"/><layoutType>single-cut</layoutType></dependencies></params></image></page-split><deskew><page id="4"><params angle="0" mode="auto"><dependencies><rotation degrees="0"/><page-outline/></dependencies></params></page><image-settings/></deskew><select-content pageDetectionTolerance="0.1"><page id="4"><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="disabled"><content-rect height="0" width="0" x="0" y="0"/><page-rect height="7015.733999999999" width="4960.62" x="0" y="0"/><content-size-mm height="-1" width="-1"/><dependencies><rotated-page-outline/><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="disabled"/></dependencies></params></page></select-content><page-layout showMiddleRect="1"><page id="4"><params autoMargins="0"><hardMarginsMM bottom="5" left="10" right="10" top="5"/><pageRect height="0" width="0" x="0" y="0"/><contentRect height="0" width="0" x="0" y="0"/><contentSizeMM height="-1" width="-1"/><alignment hor="center" null="0" vert="center"/></params></page></page-layout><output><page id="4"><zones/><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="bw"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/></page></output></filters></project>"#;
        let image = get_image(CONTENT, true);

        assert_eq!(image.layout_type, LayoutType::SingleCut);

        match image.params.pages {
            Pages::SingleCut(_) => {}
            _ => assert!(false),
        }
    }

    #[test]
    fn it_deserializes_and_validates_two_pages_pages() {
        const CONTENT: &str = r#"<project layoutDirection="LTR" outputDirectory="/home/ramzi/src/bill-organizer/out" version="3"><directories><directory id="1" path="/home/ramzi/src/bill-organizer"/></directories><files><file dirId="1" id="2" name="in.jpg"/></files><images><image fileId="2" fileImage="0" id="3" subPages="2"><size height="4961" width="7016"/><dpi horizontal="600" vertical="600"/></image></images><pages><page id="4" imageId="3" selected="selected" subPage="left"/><page id="5" imageId="3" subPage="right"/></pages><file-name-disambiguation><mapping file="2" label="0"/></file-name-disambiguation><filters><fix-orientation><image-settings/></fix-orientation><page-split defaultLayoutType="auto-detect"><image id="3" layoutType="two-pages"><params mode="auto"><pages type="two-pages"><outline><point x="0" y="0"/><point x="7016" y="0"/><point x="7016" y="4961"/><point x="0" y="4961"/><point x="0" y="0"/></outline><cutter1><p1 x="2817.82175895945" y="4962"/><p2 x="2861.124476937195" y="0"/></cutter1></pages><dependencies><rotation degrees="0"/><size height="4961" width="7016"/><layoutType>two-pages</layoutType></dependencies></params></image></page-split><deskew><image-settings/></deskew><select-content pageDetectionTolerance="0.1"/><page-layout showMiddleRect="1"/><output><page id="4"><zones/><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="bw"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/></page><page id="5"><zones/><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="bw"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/></page></output></filters></project>"#;
        let image = get_image(CONTENT, true);

        assert_eq!(image.layout_type, LayoutType::TwoPages);

        match image.params.pages {
            Pages::TwoPages(_) => {}
            _ => assert!(false),
        }
    }

    #[test]
    fn it_deserializes_split_line_mode() {
        const SINGLE_CUT_AUTO: &str = r#"<project layoutDirection="LTR" outputDirectory="/home/ramzi/src/bill-organizer/out" version="3"><directories><directory id="1" path="/home/ramzi/src/bill-organizer"/></directories><files><file dirId="1" id="2" name="in.jpg"/></files><images><image fileId="2" fileImage="0" id="3" subPages="1"><size height="4961" width="7016"/><dpi horizontal="600" vertical="600"/></image></images><pages><page id="4" imageId="3" selected="selected" subPage="single"/></pages><file-name-disambiguation><mapping file="2" label="0"/></file-name-disambiguation><filters><fix-orientation><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="193"/></page></image-settings></fix-orientation><page-split defaultLayoutType="auto-detect"><image id="3" layoutType="single-cut"><params mode="auto"><pages type="single-cut"><outline><point x="0" y="0"/><point x="7016" y="0"/><point x="7016" y="4961"/><point x="0" y="4961"/><point x="0" y="0"/></outline><cutter1><p1 x="1361.028490094375" y="0"/><p2 x="1382.679436866696" y="4962"/></cutter1><cutter2><p1 x="2817.82175895945" y="4962"/><p2 x="2861.124476937195" y="0"/></cutter2></pages><dependencies><rotation degrees="0"/><size height="4961" width="7016"/><layoutType>single-cut</layoutType></dependencies></params></image></page-split><deskew><page id="4"><params angle="0" mode="auto"><dependencies><rotation degrees="0"/><page-outline/></dependencies></params></page><image-settings/></deskew><select-content pageDetectionTolerance="0.1"><page id="4"><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="disabled"><content-rect height="0" width="0" x="0" y="0"/><page-rect height="7015.733999999999" width="4960.62" x="0" y="0"/><content-size-mm height="-1" width="-1"/><dependencies><rotated-page-outline/><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="disabled"/></dependencies></params></page></select-content><page-layout showMiddleRect="1"><page id="4"><params autoMargins="0"><hardMarginsMM bottom="5" left="10" right="10" top="5"/><pageRect height="0" width="0" x="0" y="0"/><contentRect height="0" width="0" x="0" y="0"/><contentSizeMM height="-1" width="-1"/><alignment hor="center" null="0" vert="center"/></params></page></page-layout><output><page id="4"><zones/><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="bw"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/></page></output></filters></project>"#;
        const SINGLE_CUT_MANUAL: &str = r#"<project layoutDirection="LTR" outputDirectory="/home/ramzi/src/bill-organizer/out" version="3"><directories><directory id="1" path="/home/ramzi/src/bill-organizer"/></directories><files><file dirId="1" id="2" name="in.jpg"/></files><images><image fileId="2" fileImage="0" id="3" subPages="1"><size height="4961" width="7016"/><dpi horizontal="600" vertical="600"/></image></images><pages><page id="4" imageId="3" selected="selected" subPage="single"/></pages><file-name-disambiguation><mapping file="2" label="0"/></file-name-disambiguation><filters><fix-orientation><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="193"/></page></image-settings></fix-orientation><page-split defaultLayoutType="auto-detect"><image id="3" layoutType="single-cut"><params mode="manual"><pages type="single-cut"><outline><point x="0" y="0"/><point x="7016" y="0"/><point x="7016" y="4961"/><point x="0" y="4961"/><point x="0" y="0"/></outline><cutter1><p1 x="1361.028490094375" y="0"/><p2 x="1382.679436866696" y="4962"/></cutter1><cutter2><p1 x="2817.82175895945" y="4962"/><p2 x="2861.124476937195" y="0"/></cutter2></pages><dependencies><rotation degrees="0"/><size height="4961" width="7016"/><layoutType>single-cut</layoutType></dependencies></params></image></page-split><deskew><page id="4"><params angle="0" mode="auto"><dependencies><rotation degrees="0"/><page-outline/></dependencies></params></page><image-settings/></deskew><select-content pageDetectionTolerance="0.1"><page id="4"><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="disabled"><content-rect height="0" width="0" x="0" y="0"/><page-rect height="7015.733999999999" width="4960.62" x="0" y="0"/><content-size-mm height="-1" width="-1"/><dependencies><rotated-page-outline/><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="disabled"/></dependencies></params></page></select-content><page-layout showMiddleRect="1"><page id="4"><params autoMargins="0"><hardMarginsMM bottom="5" left="10" right="10" top="5"/><pageRect height="0" width="0" x="0" y="0"/><contentRect height="0" width="0" x="0" y="0"/><contentSizeMM height="-1" width="-1"/><alignment hor="center" null="0" vert="center"/></params></page></page-layout><output><page id="4"><zones/><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="bw"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/></page></output></filters></project>"#;
        const TWO_PAGES_AUTO: &str = r#"<project layoutDirection="LTR" outputDirectory="/home/ramzi/src/bill-organizer/out" version="3"><directories><directory id="1" path="/home/ramzi/src/bill-organizer"/></directories><files><file dirId="1" id="2" name="in.jpg"/></files><images><image fileId="2" fileImage="0" id="3" subPages="2"><size height="4961" width="7016"/><dpi horizontal="600" vertical="600"/></image></images><pages><page id="4" imageId="3" selected="selected" subPage="left"/><page id="5" imageId="3" subPage="right"/></pages><file-name-disambiguation><mapping file="2" label="0"/></file-name-disambiguation><filters><fix-orientation><image-settings/></fix-orientation><page-split defaultLayoutType="auto-detect"><image id="3" layoutType="two-pages"><params mode="auto"><pages type="two-pages"><outline><point x="0" y="0"/><point x="7016" y="0"/><point x="7016" y="4961"/><point x="0" y="4961"/><point x="0" y="0"/></outline><cutter1><p1 x="2817.82175895945" y="4962"/><p2 x="2861.124476937195" y="0"/></cutter1></pages><dependencies><rotation degrees="0"/><size height="4961" width="7016"/><layoutType>two-pages</layoutType></dependencies></params></image></page-split><deskew><image-settings/></deskew><select-content pageDetectionTolerance="0.1"/><page-layout showMiddleRect="1"/><output><page id="4"><zones/><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="bw"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/></page><page id="5"><zones/><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="bw"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/></page></output></filters></project>"#;
        const TWO_PAGES_MANUAL: &str = r#"<project layoutDirection="LTR" outputDirectory="/home/ramzi/src/bill-organizer/out" version="3"><directories><directory id="1" path="/home/ramzi/src/bill-organizer"/></directories><files><file dirId="1" id="2" name="in.jpg"/></files><images><image fileId="2" fileImage="0" id="3" subPages="2"><size height="4961" width="7016"/><dpi horizontal="600" vertical="600"/></image></images><pages><page id="4" imageId="3" selected="selected" subPage="left"/><page id="5" imageId="3" subPage="right"/></pages><file-name-disambiguation><mapping file="2" label="0"/></file-name-disambiguation><filters><fix-orientation><image-settings/></fix-orientation><page-split defaultLayoutType="auto-detect"><image id="3" layoutType="two-pages"><params mode="manual"><pages type="two-pages"><outline><point x="0" y="0"/><point x="7016" y="0"/><point x="7016" y="4961"/><point x="0" y="4961"/><point x="0" y="0"/></outline><cutter1><p1 x="1361.028490094375" y="0"/><p2 x="1382.679436866696" y="4962"/></cutter1></pages><dependencies><rotation degrees="0"/><size height="4961" width="7016"/><layoutType>single-cut</layoutType></dependencies></params></image></page-split><deskew><image-settings/></deskew><select-content pageDetectionTolerance="0.1"/><page-layout showMiddleRect="1"/><output><page id="4"><zones/><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="bw"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/></page><page id="5"><zones/><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="bw"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/></page></output></filters></project>"#;

        for (content, expected_mode) in [
            (SINGLE_CUT_AUTO, Mode::Auto),
            (SINGLE_CUT_MANUAL, Mode::Manual),
            (TWO_PAGES_AUTO, Mode::Auto),
            (TWO_PAGES_MANUAL, Mode::Manual),
        ] {
            let image = get_image(content, false);
            assert_eq!(image.params.mode, expected_mode);
        }
    }
}
