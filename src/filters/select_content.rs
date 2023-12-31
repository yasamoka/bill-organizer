use serde::{Deserialize, Serialize};
use serde_with::{serde_as, BoolFromInt};
use validator::{Validate, ValidationError};

use super::{
    super::common::FloatSize,
    common::{NonNegativeRegionRect, Outline, Rect},
};

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct SelectContent {
    #[serde(rename(
        serialize = "@pageDetectionTolerance",
        deserialize = "pageDetectionTolerance"
    ))]
    pub page_detection_tolerance: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate]
    pub page: Option<Vec<Page>>,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct Page {
    #[serde(rename(serialize = "@id"))]
    pub id: u32,

    #[validate]
    pub params: PageParams,
}

#[serde_as]
#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
#[validate(schema(function = "validate_page_params"))]
pub struct PageParams {
    #[serde(rename(
        serialize = "@contentDetectionMode",
        deserialize = "contentDetectionMode"
    ))]
    pub content_detection_mode: ContentDetectionMode,

    #[serde(rename(serialize = "@fineTuneCorners", deserialize = "fineTuneCorners"))]
    #[serde_as(as = "BoolFromInt")]
    pub fine_tune_corners: bool,

    #[serde(rename(serialize = "@pageDetectionMode", deserialize = "pageDetectionMode"))]
    pub page_detection_mode: PageDetectionMode,

    #[serde(rename = "content-rect")]
    #[validate]
    pub content_rect: NonNegativeRegionRect,

    #[serde(rename = "page-rect")]
    #[validate]
    pub page_rect: Rect,

    #[serde(rename = "content-size-mm")]
    pub content_size_mm: FloatSize,

    #[validate]
    pub dependencies: Dependencies,
}

fn validate_page_params(params: &PageParams) -> Result<(), ValidationError> {
    (params.page_detection_mode == params.dependencies.params.page_detection_mode)
        .then_some(())
        .ok_or(ValidationError::new("page detection modes do not match"))?;

    (params.content_detection_mode == params.dependencies.params.content_detection_mode)
        .then_some(())
        .ok_or(ValidationError::new("content detection modes do not match"))
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum PageDetectionMode {
    Disabled,
    Auto,
    Manual,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum ContentDetectionMode {
    Disabled,
    Auto,
    Manual,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(deny_unknown_fields)]
pub struct Dependencies {
    #[serde(rename = "rotated-page-outline")]
    #[validate]
    pub rotated_page_outline: Outline,
    pub params: DependenciesParams,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct DependenciesParams {
    #[serde(rename(
        serialize = "@contentDetectionMode",
        deserialize = "contentDetectionMode"
    ))]
    pub content_detection_mode: ContentDetectionMode,
    #[serde(rename(serialize = "@fineTuneCorners", deserialize = "fineTuneCorners"))]
    #[serde_as(as = "BoolFromInt")]
    pub fine_tune_corners: bool,
    #[serde(rename(serialize = "@pageDetectionMode", deserialize = "pageDetectionMode"))]
    pub page_detection_mode: PageDetectionMode,
}

#[cfg(test)]
mod test {
    use serde_xml_rs::from_str;
    use validator::Validate;

    use crate::{
        filters::select_content::{ContentDetectionMode, Page, PageDetectionMode},
        ProjectXML,
    };

    fn get_page(content: &str) -> Page {
        let project: ProjectXML = from_str(content).unwrap();
        project.validate().unwrap();
        let mut pages = project.filters.select_content.page.unwrap();
        assert_eq!(pages.len(), 1);
        pages.pop().unwrap()
    }

    #[test]
    fn it_deserializes_and_validates_page_detection_mode() {
        const DISABLED: &str = r#"<project layoutDirection="LTR" outputDirectory="/home/ramzi/src/bill-organizer/out" version="3"><directories><directory id="1" path="/home/ramzi/src/bill-organizer"/></directories><files><file dirId="1" id="2" name="in.jpg"/></files><images><image fileId="2" fileImage="0" id="3" subPages="1"><size height="4961" width="7016"/><dpi horizontal="600" vertical="600"/></image></images><pages><page id="4" imageId="3" selected="selected" subPage="single"/></pages><file-name-disambiguation><mapping file="2" label="0"/></file-name-disambiguation><filters><fix-orientation><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="193"/></page></image-settings></fix-orientation><page-split defaultLayoutType="auto-detect"><image id="3" layoutType="single-uncut"><params mode="auto"><pages type="single-uncut"><outline><point x="0" y="0"/><point x="7016" y="0"/><point x="7016" y="4961"/><point x="0" y="4961"/><point x="0" y="0"/></outline></pages><dependencies><rotation degrees="0"/><size height="4961" width="7016"/><layoutType>single-uncut</layoutType></dependencies></params></image></page-split><deskew><page id="4"><params angle="0.125" mode="auto"><dependencies><rotation degrees="0"/><page-outline><point x="0" y="0"/><point x="7016" y="0"/><point x="7016" y="4961"/><point x="0" y="4961"/><point x="0" y="0"/></page-outline></dependencies></params></page><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="193"/></page></image-settings></deskew><select-content pageDetectionTolerance="0.1"><page id="4"><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="disabled"><content-rect height="3384" width="2260" x="344" y="388"/><page-rect height="4976.294719097549" width="7026.806517602445" x="0" y="0"/><content-size-mm height="143.256286512573" width="95.6735246803827"/><dependencies><rotated-page-outline><point x="10.82321443814471" y="0"/><point x="7026.806517602445" y="15.30652539770678"/><point x="7015.9833031643" y="4976.294719097549"/><point x="0" y="4960.988193699843"/><point x="10.82321443814471" y="0"/></rotated-page-outline><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="disabled"/></dependencies></params></page></select-content><page-layout showMiddleRect="1"><page id="4"><params autoMargins="0"><hardMarginsMM bottom="5" left="10" right="10" top="5"/><pageRect height="0" width="0" x="0" y="0"/><contentRect height="0" width="0" x="0" y="0"/><contentSizeMM height="-1" width="-1"/><alignment hor="center" null="0" vert="center"/></params></page></page-layout><output><page id="4"><zones/><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="bw"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/></page></output></filters></project>"#;
        const AUTO: &str = r#"<project layoutDirection="LTR" outputDirectory="/home/ramzi/src/bill-organizer/out" version="3"><directories><directory id="1" path="/home/ramzi/src/bill-organizer"/></directories><files><file dirId="1" id="2" name="in.jpg"/></files><images><image fileId="2" fileImage="0" id="3" subPages="1"><size height="4961" width="7016"/><dpi horizontal="600" vertical="600"/></image></images><pages><page id="4" imageId="3" selected="selected" subPage="single"/></pages><file-name-disambiguation><mapping file="2" label="0"/></file-name-disambiguation><filters><fix-orientation><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="193"/></page></image-settings></fix-orientation><page-split defaultLayoutType="auto-detect"><image id="3" layoutType="single-uncut"><params mode="auto"><pages type="single-uncut"><outline><point x="0" y="0"/><point x="7016" y="0"/><point x="7016" y="4961"/><point x="0" y="4961"/><point x="0" y="0"/></outline></pages><dependencies><rotation degrees="0"/><size height="4961" width="7016"/><layoutType>single-uncut</layoutType></dependencies></params></image></page-split><deskew><page id="4"><params angle="0.125" mode="auto"><dependencies><rotation degrees="0"/><page-outline><point x="0" y="0"/><point x="7016" y="0"/><point x="7016" y="4961"/><point x="0" y="4961"/><point x="0" y="0"/></page-outline></dependencies></params></page><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="193"/></page></image-settings></deskew><select-content pageDetectionTolerance="0.1"><page id="4"><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="auto"><content-rect height="3384" width="2260" x="344" y="388"/><page-rect height="4960" width="7024" x="4" y="16"/><content-size-mm height="143.256286512573" width="95.6735246803827"/><dependencies><rotated-page-outline><point x="10.82321443814471" y="0"/><point x="7026.806517602445" y="15.30652539770678"/><point x="7015.9833031643" y="4976.294719097549"/><point x="0" y="4960.988193699843"/><point x="10.82321443814471" y="0"/></rotated-page-outline><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="auto"/></dependencies></params></page></select-content><page-layout showMiddleRect="1"><page id="4"><params autoMargins="0"><hardMarginsMM bottom="5" left="10" right="10" top="5"/><pageRect height="0" width="0" x="0" y="0"/><contentRect height="0" width="0" x="0" y="0"/><contentSizeMM height="-1" width="-1"/><alignment hor="center" null="0" vert="center"/></params></page></page-layout><output><page id="4"><zones/><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="bw"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/></page></output></filters></project>"#;
        const MANUAL: &str = r#"<project layoutDirection="LTR" outputDirectory="/home/ramzi/src/bill-organizer/out" version="3"><directories><directory id="1" path="/home/ramzi/src/bill-organizer"/></directories><files><file dirId="1" id="2" name="in.jpg"/></files><images><image fileId="2" fileImage="0" id="3" subPages="1"><size height="4961" width="7016"/><dpi horizontal="600" vertical="600"/></image></images><pages><page id="4" imageId="3" selected="selected" subPage="single"/></pages><file-name-disambiguation><mapping file="2" label="0"/></file-name-disambiguation><filters><fix-orientation><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="193"/></page></image-settings></fix-orientation><page-split defaultLayoutType="auto-detect"><image id="3" layoutType="single-uncut"><params mode="auto"><pages type="single-uncut"><outline><point x="0" y="0"/><point x="7016" y="0"/><point x="7016" y="4961"/><point x="0" y="4961"/><point x="0" y="0"/></outline></pages><dependencies><rotation degrees="0"/><size height="4961" width="7016"/><layoutType>single-uncut</layoutType></dependencies></params></image></page-split><deskew><page id="4"><params angle="0.125" mode="auto"><dependencies><rotation degrees="0"/><page-outline><point x="0" y="0"/><point x="7016" y="0"/><point x="7016" y="4961"/><point x="0" y="4961"/><point x="0" y="0"/></page-outline></dependencies></params></page><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="193"/></page></image-settings></deskew><select-content pageDetectionTolerance="0.1"><page id="4"><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="manual"><content-rect height="3384" width="2260" x="344" y="388"/><page-rect height="4960" width="7024" x="4" y="16"/><content-size-mm height="143.256286512573" width="95.6735246803827"/><dependencies><rotated-page-outline><point x="10.82321443814471" y="0"/><point x="7026.806517602445" y="15.30652539770678"/><point x="7015.9833031643" y="4976.294719097549"/><point x="0" y="4960.988193699843"/><point x="10.82321443814471" y="0"/></rotated-page-outline><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="manual"/></dependencies></params></page></select-content><page-layout showMiddleRect="1"><page id="4"><params autoMargins="0"><hardMarginsMM bottom="5" left="10" right="10" top="5"/><pageRect height="0" width="0" x="0" y="0"/><contentRect height="0" width="0" x="0" y="0"/><contentSizeMM height="-1" width="-1"/><alignment hor="center" null="0" vert="center"/></params></page></page-layout><output><page id="4"><zones/><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="bw"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/></page></output></filters></project>"#;

        for (content, expected_mode) in [
            (DISABLED, PageDetectionMode::Disabled),
            (AUTO, PageDetectionMode::Auto),
            (MANUAL, PageDetectionMode::Manual),
        ] {
            let page = get_page(content);
            assert_eq!(page.params.page_detection_mode, expected_mode);
        }
    }

    #[test]
    fn it_deserializes_and_validates_content_detection_mode() {
        const DISABLED: &str = r#"<project layoutDirection="LTR" outputDirectory="/home/ramzi/src/bill-organizer/out" version="3"><directories><directory id="1" path="/home/ramzi/src/bill-organizer"/></directories><files><file dirId="1" id="2" name="in.jpg"/></files><images><image fileId="2" fileImage="0" id="3" subPages="1"><size height="4961" width="7016"/><dpi horizontal="600" vertical="600"/></image></images><pages><page id="4" imageId="3" selected="selected" subPage="single"/></pages><file-name-disambiguation><mapping file="2" label="0"/></file-name-disambiguation><filters><fix-orientation><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="193"/></page></image-settings></fix-orientation><page-split defaultLayoutType="auto-detect"><image id="3" layoutType="single-uncut"><params mode="auto"><pages type="single-uncut"><outline><point x="0" y="0"/><point x="7016" y="0"/><point x="7016" y="4961"/><point x="0" y="4961"/><point x="0" y="0"/></outline></pages><dependencies><rotation degrees="0"/><size height="4961" width="7016"/><layoutType>single-uncut</layoutType></dependencies></params></image></page-split><deskew><page id="4"><params angle="0.125" mode="auto"><dependencies><rotation degrees="0"/><page-outline><point x="0" y="0"/><point x="7016" y="0"/><point x="7016" y="4961"/><point x="0" y="4961"/><point x="0" y="0"/></page-outline></dependencies></params></page><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="193"/></page></image-settings></deskew><select-content pageDetectionTolerance="0.1"><page id="4"><params contentDetectionMode="disabled" fineTuneCorners="0" pageDetectionMode="disabled"><content-rect height="4976.294719097549" width="7026.806517602445" x="0" y="0"/><page-rect height="4976.294719097549" width="7026.806517602445" x="0" y="0"/><content-size-mm height="210.6635644355918" width="297.4687375159785"/><dependencies><rotated-page-outline><point x="10.82321443814471" y="0"/><point x="7026.806517602445" y="15.30652539770678"/><point x="7015.9833031643" y="4976.294719097549"/><point x="0" y="4960.988193699843"/><point x="10.82321443814471" y="0"/></rotated-page-outline><params contentDetectionMode="disabled" fineTuneCorners="0" pageDetectionMode="disabled"/></dependencies></params></page></select-content><page-layout showMiddleRect="1"><page id="4"><params autoMargins="0"><hardMarginsMM bottom="5" left="10" right="10" top="5"/><pageRect height="0" width="0" x="0" y="0"/><contentRect height="0" width="0" x="0" y="0"/><contentSizeMM height="-1" width="-1"/><alignment hor="center" null="0" vert="center"/></params></page></page-layout><output><page id="4"><zones/><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="bw"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/></page></output></filters></project>"#;
        const AUTO: &str = r#"<project layoutDirection="LTR" outputDirectory="/home/ramzi/src/bill-organizer/out" version="3"><directories><directory id="1" path="/home/ramzi/src/bill-organizer"/></directories><files><file dirId="1" id="2" name="in.jpg"/></files><images><image fileId="2" fileImage="0" id="3" subPages="1"><size height="4961" width="7016"/><dpi horizontal="600" vertical="600"/></image></images><pages><page id="4" imageId="3" selected="selected" subPage="single"/></pages><file-name-disambiguation><mapping file="2" label="0"/></file-name-disambiguation><filters><fix-orientation><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="193"/></page></image-settings></fix-orientation><page-split defaultLayoutType="auto-detect"><image id="3" layoutType="single-uncut"><params mode="auto"><pages type="single-uncut"><outline><point x="0" y="0"/><point x="7016" y="0"/><point x="7016" y="4961"/><point x="0" y="4961"/><point x="0" y="0"/></outline></pages><dependencies><rotation degrees="0"/><size height="4961" width="7016"/><layoutType>single-uncut</layoutType></dependencies></params></image></page-split><deskew><page id="4"><params angle="0.125" mode="auto"><dependencies><rotation degrees="0"/><page-outline><point x="0" y="0"/><point x="7016" y="0"/><point x="7016" y="4961"/><point x="0" y="4961"/><point x="0" y="0"/></page-outline></dependencies></params></page><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="193"/></page></image-settings></deskew><select-content pageDetectionTolerance="0.1"><page id="4"><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="disabled"><content-rect height="3384" width="2260" x="344" y="388"/><page-rect height="4976.294719097549" width="7026.806517602445" x="0" y="0"/><content-size-mm height="143.256286512573" width="95.6735246803827"/><dependencies><rotated-page-outline><point x="10.82321443814471" y="0"/><point x="7026.806517602445" y="15.30652539770678"/><point x="7015.9833031643" y="4976.294719097549"/><point x="0" y="4960.988193699843"/><point x="10.82321443814471" y="0"/></rotated-page-outline><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="disabled"/></dependencies></params></page></select-content><page-layout showMiddleRect="1"><page id="4"><params autoMargins="0"><hardMarginsMM bottom="5" left="10" right="10" top="5"/><pageRect height="0" width="0" x="0" y="0"/><contentRect height="0" width="0" x="0" y="0"/><contentSizeMM height="-1" width="-1"/><alignment hor="center" null="0" vert="center"/></params></page></page-layout><output><page id="4"><zones/><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="bw"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/></page></output></filters></project>"#;
        const MANUAL: &str = r#"<project layoutDirection="LTR" outputDirectory="/home/ramzi/src/bill-organizer/out" version="3"><directories><directory id="1" path="/home/ramzi/src/bill-organizer"/></directories><files><file dirId="1" id="2" name="in.jpg"/></files><images><image fileId="2" fileImage="0" id="3" subPages="1"><size height="4961" width="7016"/><dpi horizontal="600" vertical="600"/></image></images><pages><page id="4" imageId="3" selected="selected" subPage="single"/></pages><file-name-disambiguation><mapping file="2" label="0"/></file-name-disambiguation><filters><fix-orientation><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="193"/></page></image-settings></fix-orientation><page-split defaultLayoutType="auto-detect"><image id="3" layoutType="single-uncut"><params mode="auto"><pages type="single-uncut"><outline><point x="0" y="0"/><point x="7016" y="0"/><point x="7016" y="4961"/><point x="0" y="4961"/><point x="0" y="0"/></outline></pages><dependencies><rotation degrees="0"/><size height="4961" width="7016"/><layoutType>single-uncut</layoutType></dependencies></params></image></page-split><deskew><page id="4"><params angle="0.125" mode="auto"><dependencies><rotation degrees="0"/><page-outline><point x="0" y="0"/><point x="7016" y="0"/><point x="7016" y="4961"/><point x="0" y="4961"/><point x="0" y="0"/></page-outline></dependencies></params></page><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="193"/></page></image-settings></deskew><select-content pageDetectionTolerance="0.1"><page id="4"><params contentDetectionMode="manual" fineTuneCorners="0" pageDetectionMode="disabled"><content-rect height="4976.294719097549" width="7026.806517602445" x="0" y="0"/><page-rect height="4976.294719097549" width="7026.806517602445" x="0" y="0"/><content-size-mm height="210.6635644355918" width="297.4687375159785"/><dependencies><rotated-page-outline><point x="10.82321443814471" y="0"/><point x="7026.806517602445" y="15.30652539770678"/><point x="7015.9833031643" y="4976.294719097549"/><point x="0" y="4960.988193699843"/><point x="10.82321443814471" y="0"/></rotated-page-outline><params contentDetectionMode="manual" fineTuneCorners="0" pageDetectionMode="disabled"/></dependencies></params></page></select-content><page-layout showMiddleRect="1"><page id="4"><params autoMargins="0"><hardMarginsMM bottom="5" left="10" right="10" top="5"/><pageRect height="0" width="0" x="0" y="0"/><contentRect height="0" width="0" x="0" y="0"/><contentSizeMM height="-1" width="-1"/><alignment hor="center" null="0" vert="center"/></params></page></page-layout><output><page id="4"><zones/><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="bw"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/></page></output></filters></project>"#;

        for (content, expected_mode) in [
            (DISABLED, ContentDetectionMode::Disabled),
            (AUTO, ContentDetectionMode::Auto),
            (MANUAL, ContentDetectionMode::Manual),
        ] {
            let page = get_page(content);
            assert_eq!(page.params.content_detection_mode, expected_mode);
        }
    }
}
