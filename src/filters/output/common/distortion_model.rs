use std::fmt::Debug;

use serde::{de, Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as};

use super::super::super::super::common::Point;

// ScanTailor seems to leave in a distortion-model
// if dewarping was enabled at some point, even if
// it is now off, without cleaning up

#[derive(Serialize, Debug)]
pub enum DistortionModel {
    Auto(AutoDistortionModel),
    Marginal(MarginalDistortionModel),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct AutoDistortionModel {
    #[serde(rename = "top-curve")]
    pub top_curve: PolylineCurve,

    #[serde(rename = "bottom-curve")]
    pub bottom_curve: PolylineCurve,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct MarginalDistortionModel {
    #[serde(rename = "top-curve")]
    pub top_curve: XSplineAndPolylineCurve,

    #[serde(rename = "bottom-curve")]
    pub bottom_curve: XSplineAndPolylineCurve,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct XSplineAndPolylineCurve {
    pub xspline: XSpline,

    #[serde_as(as = "Base64")]
    pub polyline: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct XSpline {
    point: Vec<Point>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PolylineCurve {
    pub polyline: Vec<u8>,
}

impl<'de> Deserialize<'de> for DistortionModel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Serialize, Deserialize, Debug)]
        #[serde(deny_unknown_fields)]
        pub struct Model {
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

        let m = Model::deserialize(deserializer)?;

        match (
            m.top_curve.xspline,
            m.top_curve.polyline,
            m.bottom_curve.xspline,
            m.bottom_curve.polyline,
        ) {
            (
                Some(top_xspline),
                Some(top_polyline),
                Some(bottom_xspline),
                Some(bottom_polyline),
            ) => Ok(DistortionModel::Marginal(MarginalDistortionModel {
                top_curve: XSplineAndPolylineCurve {
                    xspline: top_xspline,
                    polyline: top_polyline,
                },
                bottom_curve: XSplineAndPolylineCurve {
                    xspline: bottom_xspline,
                    polyline: bottom_polyline,
                },
            })),
            (None, Some(top_polyline), None, Some(bottom_polyline)) => {
                Ok(DistortionModel::Auto(AutoDistortionModel {
                    top_curve: PolylineCurve {
                        polyline: top_polyline,
                    },
                    bottom_curve: PolylineCurve {
                        polyline: bottom_polyline,
                    },
                }))
            }
            _ => Err(de::Error::custom("Unknown Curve variant")),
        }
    }
}

#[cfg(test)]
mod test {
    use serde_xml_rs::from_str;

    use crate::ProjectXML;

    use super::DistortionModel;

    fn get_distortion_model(content: &str) -> DistortionModel {
        let project: ProjectXML = from_str(content).unwrap();
        let mut pages = project.filters.output.page;
        assert_eq!(pages.len(), 1);
        let page = pages.pop().unwrap();
        page.params.distortion_model.unwrap()
    }

    #[test]
    fn it_deserializes_auto_distortion_model() {
        const CONTENT: &str = r#"<project layoutDirection="LTR" outputDirectory="/home/ramzi/src/bill-organizer/out" version="3"><directories><directory id="1" path="/home/ramzi/src/bill-organizer"/></directories><files><file dirId="1" id="2" name="in.jpg"/></files><images><image fileId="2" fileImage="0" id="3" subPages="1"><size height="4961" width="7016"/><dpi horizontal="600" vertical="600"/></image></images><pages><page id="4" imageId="3" selected="selected" subPage="single"/></pages><file-name-disambiguation><mapping file="2" label="0"/></file-name-disambiguation><filters><fix-orientation><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="193"/></page></image-settings></fix-orientation><page-split defaultLayoutType="auto-detect"><image id="3" layoutType="single-uncut"><params mode="auto"><pages type="single-uncut"><outline><point x="0" y="0"/><point x="7016" y="0"/><point x="7016" y="4961"/><point x="0" y="4961"/><point x="0" y="0"/></outline></pages><dependencies><rotation degrees="0"/><size height="4961" width="7016"/><layoutType>single-uncut</layoutType></dependencies></params></image></page-split><deskew><page id="4"><params angle="0.125" mode="auto"><dependencies><rotation degrees="0"/><page-outline><point x="0" y="0"/><point x="7016" y="0"/><point x="7016" y="4961"/><point x="0" y="4961"/><point x="0" y="0"/></page-outline></dependencies></params></page><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="193"/></page></image-settings></deskew><select-content pageDetectionTolerance="0.1"><page id="4"><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="disabled"><content-rect height="3384" width="2260" x="344" y="388"/><page-rect height="4976.294719097549" width="7026.806517602445" x="0" y="0"/><content-size-mm height="143.256286512573" width="95.6735246803827"/><dependencies><rotated-page-outline><point x="10.82321443814471" y="0"/><point x="7026.806517602445" y="15.30652539770678"/><point x="7015.9833031643" y="4976.294719097549"/><point x="0" y="4960.988193699843"/><point x="10.82321443814471" y="0"/></rotated-page-outline><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="disabled"/></dependencies></params></page></select-content><page-layout showMiddleRect="1"><page id="4"><params autoMargins="0"><hardMarginsMM bottom="5" left="10" right="10" top="5"/><pageRect height="4976.294719097549" width="7026.806517602445" x="0" y="0"/><contentRect height="3384" width="2260" x="344" y="388"/><contentSizeMM height="143.256286512573" width="95.6735246803827"/><alignment hor="center" null="0" vert="center"/></params></page></page-layout><output><page id="4"><zones/><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><distortion-model><top-curve><polyline>4QKnQ9eiwUNGICJFuyu/Qw==</polyline></top-curve><bottom-curve><polyline>3rOqQzq0a0VmliJFVmVrRQ==</polyline></bottom-curve></distortion-model><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="auto" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="bw"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/><output-params><image blackOnWhite="1" depthPerception="2" despeckleLevel="1"><size height="3620" width="2732"/><content-rect height="3384" width="2260" x="236" y="118"/><crop-area><point x="-97.17678556185528" y="-270"/><point x="6918.806517602445" y="-254.6934746022932"/><point x="6907.9833031643" y="4706.294719097549"/><point x="-108" y="4690.988193699843"/><point x="-97.17678556185528" y="-270"/></crop-area><partial-xform><m11>0.9999976201773518</m11><m12>0.00218165983433677</m12><m21>-0.00218165983433677</m21><m22>0.9999976201773518</m22></partial-xform><dpi horizontal="600" vertical="600"/><color-params colorMode="bw"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><distortion-model><top-curve><polyline>4QKnQ9eiwUNGICJFuyu/Qw==</polyline></top-curve><bottom-curve><polyline>3rOqQzq0a0VmliJFVmVrRQ==</polyline></bottom-curve></distortion-model><dewarping-options mode="auto" postDeskew="1" postDeskewAngle="0"/><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/></image><source_file mtime="1698783384" size="2890349"/><file mtime="1699404572" size="26508"/><speckles mtime="1699404572" size="692"/><zones/><fill-zones/></output-params></page></output></filters></project>"#;
        let distortion_model = get_distortion_model(CONTENT);
        match distortion_model {
            DistortionModel::Auto(_) => {}
            _ => assert!(false),
        }
    }

    #[test]
    fn it_deserializes_marginal_distortion_model() {
        const CONTENT: &str = r#"<project layoutDirection="LTR" outputDirectory="/home/ramzi/src/bill-organizer/out" version="3"><directories><directory id="1" path="/home/ramzi/src/bill-organizer"/></directories><files><file dirId="1" id="2" name="in.jpg"/></files><images><image fileId="2" fileImage="0" id="3" subPages="1"><size height="4961" width="7016"/><dpi horizontal="600" vertical="600"/></image></images><pages><page id="4" imageId="3" selected="selected" subPage="single"/></pages><file-name-disambiguation><mapping file="2" label="0"/></file-name-disambiguation><filters><fix-orientation><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="193"/></page></image-settings></fix-orientation><page-split defaultLayoutType="auto-detect"><image id="3" layoutType="single-uncut"><params mode="auto"><pages type="single-uncut"><outline><point x="0" y="0"/><point x="7016" y="0"/><point x="7016" y="4961"/><point x="0" y="4961"/><point x="0" y="0"/></outline></pages><dependencies><rotation degrees="0"/><size height="4961" width="7016"/><layoutType>single-uncut</layoutType></dependencies></params></image></page-split><deskew><page id="4"><params angle="0.125" mode="auto"><dependencies><rotation degrees="0"/><page-outline><point x="0" y="0"/><point x="7016" y="0"/><point x="7016" y="4961"/><point x="0" y="4961"/><point x="0" y="0"/></page-outline></dependencies></params></page><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="193"/></page></image-settings></deskew><select-content pageDetectionTolerance="0.1"><page id="4"><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="disabled"><content-rect height="3384" width="2260" x="344" y="388"/><page-rect height="4976.294719097549" width="7026.806517602445" x="0" y="0"/><content-size-mm height="143.256286512573" width="95.6735246803827"/><dependencies><rotated-page-outline><point x="10.82321443814471" y="0"/><point x="7026.806517602445" y="15.30652539770678"/><point x="7015.9833031643" y="4976.294719097549"/><point x="0" y="4960.988193699843"/><point x="10.82321443814471" y="0"/></rotated-page-outline><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="disabled"/></dependencies></params></page></select-content><page-layout showMiddleRect="1"><page id="4"><params autoMargins="0"><hardMarginsMM bottom="5" left="10" right="10" top="5"/><pageRect height="4976.294719097549" width="7026.806517602445" x="0" y="0"/><contentRect height="3384" width="2260" x="344" y="388"/><contentSizeMM height="143.256286512573" width="95.6735246803827"/><alignment hor="center" null="0" vert="center"/></params></page></page-layout><output><page id="4"><zones/><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><distortion-model><top-curve><xspline><point x="334.0224766759178" y="14"/><point x="2204.362853173144" y="0"/><point x="2282.293702193861" y="0"/><point x="2360.224551214579" y="0"/><point x="2438.155400235297" y="0"/><point x="2516.086249256015" y="0"/><point x="2594.017098276733" y="0"/></xspline><polyline>4QKnQwAAYEEYNO5EVVUVQLOkDkUAAAAAmIMTRQAAAAB9YhhFAAAAAGFBHUUAAAAARiAiRQAAAAA=</polyline></top-curve><bottom-curve><xspline><point x="341.4052135553135" y="4960"/><point x="2204.362853173144" y="4960"/><point x="2282.293702193861" y="4960"/><point x="2360.224551214579" y="4960"/><point x="2438.155400235297" y="4960"/><point x="2516.086249256015" y="4960"/><point x="2601.399835156129" y="4960"/></xspline><polyline>3rOqQwAAm0V4W+5EAACbRbOkDkUAAJtFmIMTRQAAm0V9YhhFAACbRRFVHUUAAJtFZpYiRQAAm0U=</polyline></bottom-curve></distortion-model><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="marginal" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="bw"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/><output-params><image blackOnWhite="1" depthPerception="2" despeckleLevel="1"><size height="3620" width="2732"/><content-rect height="3384" width="2260" x="236" y="118"/><crop-area><point x="-97.17678556185528" y="-270"/><point x="6918.806517602445" y="-254.6934746022932"/><point x="6907.9833031643" y="4706.294719097549"/><point x="-108" y="4690.988193699843"/><point x="-97.17678556185528" y="-270"/></crop-area><partial-xform><m11>0.9999976201773518</m11><m12>0.00218165983433677</m12><m21>-0.00218165983433677</m21><m22>0.9999976201773518</m22></partial-xform><dpi horizontal="600" vertical="600"/><color-params colorMode="bw"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><distortion-model><top-curve><xspline><point x="334.0224766759178" y="14"/><point x="2204.362853173144" y="0"/><point x="2282.293702193861" y="0"/><point x="2360.224551214579" y="0"/><point x="2438.155400235297" y="0"/><point x="2516.086249256015" y="0"/><point x="2594.017098276733" y="0"/></xspline><polyline>4QKnQwAAYEEYNO5EVVUVQLOkDkUAAAAAmIMTRQAAAAB9YhhFAAAAAGFBHUUAAAAARiAiRQAAAAA=</polyline></top-curve><bottom-curve><xspline><point x="341.4052135553135" y="4960"/><point x="2204.362853173144" y="4960"/><point x="2282.293702193861" y="4960"/><point x="2360.224551214579" y="4960"/><point x="2438.155400235297" y="4960"/><point x="2516.086249256015" y="4960"/><point x="2601.399835156129" y="4960"/></xspline><polyline>3rOqQwAAm0V4W+5EAACbRbOkDkUAAJtFmIMTRQAAm0V9YhhFAACbRRFVHUUAAJtFZpYiRQAAm0U=</polyline></bottom-curve></distortion-model><dewarping-options mode="marginal" postDeskew="1" postDeskewAngle="0"/><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/></image><source_file mtime="1698783384" size="2890349"/><file mtime="1699405212" size="26562"/><speckles mtime="1699405212" size="690"/><zones/><fill-zones/></output-params></page></output></filters></project>"#;
        let distortion_model = get_distortion_model(CONTENT);
        match distortion_model {
            DistortionModel::Marginal(_) => {}
            _ => assert!(false),
        }
    }
}
