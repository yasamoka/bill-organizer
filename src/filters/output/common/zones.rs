use serde::{Deserialize, Serialize};

use super::super::super::common::Outline;

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Zones {
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<Vec<Zone>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Zone {
    pub spline: Outline,
    pub properties: Properties,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Properties {
    property: Vec<Property>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", deny_unknown_fields)]
pub enum Property {
    #[serde(rename = "PictureZoneProperty")]
    PictureZone(PictureZoneProperty),
    #[serde(rename = "ZoneCategoryProperty")]
    ZoneCategory(ZoneCategoryProperty),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PictureZoneProperty {
    layer: Layer,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Layer {
    Painter2,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ZoneCategoryProperty {
    #[serde(rename(serialize = "@zoneCategory", deserialize = "zoneCategory"))]
    category: ZoneCategory,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ZoneCategory {
    Auto,
}

#[cfg(test)]
mod test {
    use serde_xml_rs::from_str;

    use crate::{filters::output::common::zones::Property, ProjectXML};

    #[test]
    fn it_deserializes_property() {
        const CONTENT: &str = r#"<project layoutDirection="LTR" outputDirectory="/home/ramzi/src/bill-organizer/out" version="3"><directories><directory id="1" path="/home/ramzi/src/bill-organizer"/></directories><files><file dirId="1" id="2" name="in.jpg"/></files><images><image fileId="2" fileImage="0" id="3" subPages="1"><size height="4961" width="7016"/><dpi horizontal="600" vertical="600"/></image></images><pages><page id="4" imageId="3" selected="selected" subPage="single"/></pages><file-name-disambiguation><mapping file="2" label="0"/></file-name-disambiguation><filters><fix-orientation><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="193"/></page></image-settings></fix-orientation><page-split defaultLayoutType="auto-detect"><image id="3" layoutType="single-uncut"><params mode="auto"><pages type="single-uncut"><outline><point x="0" y="0"/><point x="7016" y="0"/><point x="7016" y="4961"/><point x="0" y="4961"/><point x="0" y="0"/></outline></pages><dependencies><rotation degrees="0"/><size height="4961" width="7016"/><layoutType>single-uncut</layoutType></dependencies></params></image></page-split><deskew><page id="4"><params angle="0.125" mode="auto"><dependencies><rotation degrees="0"/><page-outline><point x="0" y="0"/><point x="7016" y="0"/><point x="7016" y="4961"/><point x="0" y="4961"/><point x="0" y="0"/></page-outline></dependencies></params></page><image-settings><page id="4"><image-params blackOnWhite="1" bwThreshold="193"/></page></image-settings></deskew><select-content pageDetectionTolerance="0.1"><page id="4"><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="disabled"><content-rect height="3384" width="2260" x="344" y="388"/><page-rect height="4976.294719097549" width="7026.806517602445" x="0" y="0"/><content-size-mm height="143.256286512573" width="95.6735246803827"/><dependencies><rotated-page-outline><point x="10.82321443814471" y="0"/><point x="7026.806517602445" y="15.30652539770678"/><point x="7015.9833031643" y="4976.294719097549"/><point x="0" y="4960.988193699843"/><point x="10.82321443814471" y="0"/></rotated-page-outline><params contentDetectionMode="auto" fineTuneCorners="0" pageDetectionMode="disabled"/></dependencies></params></page></select-content><page-layout showMiddleRect="1"><page id="4"><params autoMargins="0"><hardMarginsMM bottom="5" left="10" right="10" top="5"/><pageRect height="4976.294719097549" width="7026.806517602445" x="0" y="0"/><contentRect height="3384" width="2260" x="344" y="388"/><contentSizeMM height="143.256286512573" width="95.6735246803827"/><alignment hor="center" null="0" vert="center"/></params></page></page-layout><output><page id="4"><zones><zone><spline><point x="1296.024550606199" y="387.1734366977415"/><point x="1303.285114534872" y="3715.165516647968"/><point x="1997.283462937954" y="3713.651444722938"/><point x="1990.022899009281" y="385.6593647727118"/><point x="1296.024550606199" y="387.1734366977415"/></spline><properties><property type="ZoneCategoryProperty" zoneCategory="auto"/><property layer="painter2" type="PictureZoneProperty"/></properties></zone><zone><spline><point x="666.8229425617324" y="1670.549194780407"/><point x="668.7340765766114" y="2546.547110055767"/><point x="946.7334149859151" y="2545.940608621822"/><point x="944.8222809710362" y="1669.942693346462"/><point x="666.8229425617324" y="1670.549194780407"/></spline><properties><property type="ZoneCategoryProperty" zoneCategory="auto"/><property layer="painter2" type="PictureZoneProperty"/></properties></zone></zones><fill-zones/><params blackOnWhite="1" depthPerception="2" despeckleLevel="1"><picture-shape-options higherSearchSensitivity="1" pictureShape="rectangular" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><dpi horizontal="600" vertical="600"/><color-params colorMode="mixed"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/></params><processing-params autoZonesFound="1" blackOnWhiteSetManually="0"/><output-params><image blackOnWhite="1" depthPerception="2" despeckleLevel="1"><size height="3620" width="2732"/><content-rect height="3384" width="2260" x="236" y="118"/><crop-area><point x="-97.17678556185528" y="-270"/><point x="6918.806517602445" y="-254.6934746022932"/><point x="6907.9833031643" y="4706.294719097549"/><point x="-108" y="4690.988193699843"/><point x="-97.17678556185528" y="-270"/></crop-area><partial-xform><m11>0.9999976201773518</m11><m12>0.00218165983433677</m12><m21>-0.00218165983433677</m21><m22>0.9999976201773518</m22></partial-xform><dpi horizontal="600" vertical="600"/><color-params colorMode="mixed"><color-or-grayscale fillMargins="1" fillOffcut="1" fillingColor="background" normalizeIlluminationColor="0"><posterization-options enabled="0" forceBlackAndWhite="1" level="4" normalizationEnabled="0"/></color-or-grayscale><bw binarizationMethod="otsu" morphologicalSmoothing="1" normalizeIlluminationBW="1" sauvolaCoef="0.34" savitzkyGolaySmoothing="1" thresholdAdj="0" windowSize="200" wolfCoef="0.3" wolfLowerBound="1" wolfUpperBound="254"><color-segmenter-options blueThresholdAdjustment="0" enabled="0" greenThresholdAdjustment="0" noiseReduction="7" redThresholdAdjustment="0"/></bw></color-params><splitting originalBackground="0" splitOutput="0" splittingMode="bw"/><picture-shape-options higherSearchSensitivity="1" pictureShape="rectangular" sensitivity="100"/><dewarping-options mode="off" postDeskew="1" postDeskewAngle="0"/><processing-params autoZonesFound="1" blackOnWhiteSetManually="0"/></image><source_file mtime="1698783384" size="2890349"/><file mtime="1699402381" size="5426746"/><automask mtime="1699402381" size="1100"/><speckles mtime="1699402381" size="690"/><zones><zone><spline><point x="1296.024550606199" y="387.1734366977415"/><point x="1303.285114534872" y="3715.165516647968"/><point x="1997.283462937954" y="3713.651444722938"/><point x="1990.022899009281" y="385.6593647727118"/><point x="1296.024550606199" y="387.1734366977415"/></spline><properties><property layer="painter2" type="PictureZoneProperty"/><property type="ZoneCategoryProperty" zoneCategory="auto"/></properties></zone><zone><spline><point x="666.8229425617324" y="1670.549194780407"/><point x="668.7340765766114" y="2546.547110055767"/><point x="946.7334149859151" y="2545.940608621822"/><point x="944.8222809710362" y="1669.942693346462"/><point x="666.8229425617324" y="1670.549194780407"/></spline><properties><property layer="painter2" type="PictureZoneProperty"/><property type="ZoneCategoryProperty" zoneCategory="auto"/></properties></zone></zones><fill-zones/></output-params></page></output></filters></project>"#;
        let project: ProjectXML = from_str(CONTENT).unwrap();

        let mut pages = project.filters.output.page;
        assert_eq!(pages.len(), 1);
        let page = pages.pop().unwrap();

        let zones = page.zones.zone.unwrap();
        assert_eq!(zones.len(), 2);
        let zone = zones.first().unwrap();

        let properties = &zone.properties.property;
        assert_eq!(properties.len(), 2);
        if let Property::ZoneCategory(_) = properties[0] {
            assert!(true);
        }
        if let Property::PictureZone(_) = properties[1] {
            assert!(true);
        }
    }
}
