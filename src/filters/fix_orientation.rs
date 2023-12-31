use serde::{Deserialize, Serialize};

use super::common::ImageSettings;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct FixOrientation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Vec<Image>>,

    #[serde(rename = "image-settings")]
    pub image_settings: ImageSettings,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Image {
    #[serde(rename(serialize = "@id"))]
    pub id: u32,

    pub rotation: Rotation,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Rotation {
    #[serde(rename(serialize = "@degrees"))]
    pub degrees: Degrees,
}

// clockwise
#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub enum Degrees {
    #[serde(rename = "90")]
    _90,
    #[serde(rename = "180")]
    _180,
    #[serde(rename = "270")]
    _270,
}

#[cfg(test)]
mod test {
    use serde_xml_rs::from_str;

    use crate::{filters::fix_orientation::Degrees, ProjectXML};

    #[test]
    fn it_deserializes_no_rotation() {
        const CONTENT: &str = "<project layoutDirection=\"LTR\" outputDirectory=\"/home/ramzi/src/bill-organizer/out\" version=\"3\"><directories><directory id=\"1\" path=\"/home/ramzi/src/bill-organizer\"/></directories><files><file dirId=\"1\" id=\"2\" name=\"in.jpg\"/></files><images><image fileId=\"2\" fileImage=\"0\" id=\"3\" subPages=\"2\"><size height=\"4961\" width=\"7016\"/><dpi horizontal=\"600\" vertical=\"600\"/></image></images><pages><page id=\"4\" imageId=\"3\" selected=\"selected\" subPage=\"left\"/><page id=\"5\" imageId=\"3\" subPage=\"right\"/></pages><file-name-disambiguation><mapping file=\"2\" label=\"0\"/></file-name-disambiguation><filters><fix-orientation><image-settings/></fix-orientation><page-split defaultLayoutType=\"auto-detect\"/><deskew><image-settings/></deskew><select-content pageDetectionTolerance=\"0.1\"/><page-layout showMiddleRect=\"1\"/><output><page id=\"4\"><zones/><fill-zones/><params blackOnWhite=\"1\" depthPerception=\"2\" despeckleLevel=\"1\"><picture-shape-options higherSearchSensitivity=\"0\" pictureShape=\"free\" sensitivity=\"100\"/><dewarping-options mode=\"off\" postDeskew=\"1\" postDeskewAngle=\"0\"/><dpi horizontal=\"600\" vertical=\"600\"/><color-params colorMode=\"bw\"><color-or-grayscale fillMargins=\"1\" fillOffcut=\"1\" fillingColor=\"background\" normalizeIlluminationColor=\"0\"><posterization-options enabled=\"0\" forceBlackAndWhite=\"1\" level=\"4\" normalizationEnabled=\"0\"/></color-or-grayscale><bw binarizationMethod=\"otsu\" morphologicalSmoothing=\"1\" normalizeIlluminationBW=\"1\" sauvolaCoef=\"0.34\" savitzkyGolaySmoothing=\"1\" thresholdAdj=\"0\" windowSize=\"200\" wolfCoef=\"0.3\" wolfLowerBound=\"1\" wolfUpperBound=\"254\"><color-segmenter-options blueThresholdAdjustment=\"0\" enabled=\"0\" greenThresholdAdjustment=\"0\" noiseReduction=\"7\" redThresholdAdjustment=\"0\"/></bw></color-params><splitting originalBackground=\"0\" splitOutput=\"0\" splittingMode=\"bw\"/></params><processing-params autoZonesFound=\"0\" blackOnWhiteSetManually=\"0\"/></page><page id=\"5\"><zones/><fill-zones/><params blackOnWhite=\"1\" depthPerception=\"2\" despeckleLevel=\"1\"><picture-shape-options higherSearchSensitivity=\"0\" pictureShape=\"free\" sensitivity=\"100\"/><dewarping-options mode=\"off\" postDeskew=\"1\" postDeskewAngle=\"0\"/><dpi horizontal=\"600\" vertical=\"600\"/><color-params colorMode=\"bw\"><color-or-grayscale fillMargins=\"1\" fillOffcut=\"1\" fillingColor=\"background\" normalizeIlluminationColor=\"0\"><posterization-options enabled=\"0\" forceBlackAndWhite=\"1\" level=\"4\" normalizationEnabled=\"0\"/></color-or-grayscale><bw binarizationMethod=\"otsu\" morphologicalSmoothing=\"1\" normalizeIlluminationBW=\"1\" sauvolaCoef=\"0.34\" savitzkyGolaySmoothing=\"1\" thresholdAdj=\"0\" windowSize=\"200\" wolfCoef=\"0.3\" wolfLowerBound=\"1\" wolfUpperBound=\"254\"><color-segmenter-options blueThresholdAdjustment=\"0\" enabled=\"0\" greenThresholdAdjustment=\"0\" noiseReduction=\"7\" redThresholdAdjustment=\"0\"/></bw></color-params><splitting originalBackground=\"0\" splitOutput=\"0\" splittingMode=\"bw\"/></params><processing-params autoZonesFound=\"0\" blackOnWhiteSetManually=\"0\"/></page></output></filters></project>";

        let project: ProjectXML = from_str(CONTENT).unwrap();
        assert!(project.filters.fix_orientation.image.is_none());
    }

    #[test]
    fn it_deserializes_rotation() {
        const _90: &str = r#"<project layoutDirection="LTR" outputDirectory="/home/ramzi/src/bill-organizer/out" version="3"><directories><directory id="1" path="/home/ramzi/src/bill-organizer"/></directories><files><file dirId="1" id="2" name="in.jpg"/></files><images><image fileId="2" fileImage="0" id="3" subPages="2"><size height="4961" width="7016"/><dpi horizontal="600" vertical="600"/></image></images><pages><page id="4" imageId="3" selected="selected" subPage="left"/><page id="5" imageId="3" subPage="right"/></pages><file-name-disambiguation><mapping file="2" label="0"/></file-name-disambiguation><filters><fix-orientation><image id="3"><rotation degrees="90"/></image><image-settings/></fix-orientation><page-split defaultLayoutType="auto-detect"/><deskew><image-settings/></deskew><select-content pageDetectionTolerance="0.1"/><page-layout showMiddleRect="1"/><output><page id="4"><zones/><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="bw"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/></page><page id="5"><zones/><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="bw"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/></page></output></filters></project>"#;
        const _180: &str = r#"<project layoutDirection="LTR" outputDirectory="/home/ramzi/src/bill-organizer/out" version="3"><directories><directory id="1" path="/home/ramzi/src/bill-organizer"/></directories><files><file dirId="1" id="2" name="in.jpg"/></files><images><image fileId="2" fileImage="0" id="3" subPages="2"><size height="4961" width="7016"/><dpi horizontal="600" vertical="600"/></image></images><pages><page id="4" imageId="3" selected="selected" subPage="left"/><page id="5" imageId="3" subPage="right"/></pages><file-name-disambiguation><mapping file="2" label="0"/></file-name-disambiguation><filters><fix-orientation><image id="3"><rotation degrees="180"/></image><image-settings/></fix-orientation><page-split defaultLayoutType="auto-detect"/><deskew><image-settings/></deskew><select-content pageDetectionTolerance="0.1"/><page-layout showMiddleRect="1"/><output><page id="4"><zones/><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="bw"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/></page><page id="5"><zones/><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="bw"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/></page></output></filters></project>"#;
        const _270: &str = r#"<project layoutDirection="LTR" outputDirectory="/home/ramzi/src/bill-organizer/out" version="3"><directories><directory id="1" path="/home/ramzi/src/bill-organizer"/></directories><files><file dirId="1" id="2" name="in.jpg"/></files><images><image fileId="2" fileImage="0" id="3" subPages="2"><size height="4961" width="7016"/><dpi horizontal="600" vertical="600"/></image></images><pages><page id="4" imageId="3" selected="selected" subPage="left"/><page id="5" imageId="3" subPage="right"/></pages><file-name-disambiguation><mapping file="2" label="0"/></file-name-disambiguation><filters><fix-orientation><image id="3"><rotation degrees="270"/></image><image-settings/></fix-orientation><page-split defaultLayoutType="auto-detect"/><deskew><image-settings/></deskew><select-content pageDetectionTolerance="0.1"/><page-layout showMiddleRect="1"/><output><page id="4"><zones/><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="bw"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/></page><page id="5"><zones/><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><picture-shape-options higherSearchSensitivity="0" pictureShape="free" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="bw"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="0" blackOnWhiteSetManually="0"/></page></output></filters></project>"#;

        for (content, expected_degrees) in [
            (_90, Degrees::_90),
            (_180, Degrees::_180),
            (_270, Degrees::_270),
        ] {
            let project: ProjectXML = from_str(content).unwrap();

            let images = project.filters.fix_orientation.image.unwrap();
            assert_eq!(images.len(), 1);

            let image = images.first().unwrap();
            assert_eq!(image.rotation.degrees, expected_degrees);
        }
    }
}
