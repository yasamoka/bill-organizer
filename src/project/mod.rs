mod common;
mod directories;
mod file_name_disambiguation;
mod files;
mod filters;
mod images;
mod layout_direction;
mod main;
mod pages;

pub use main::ProjectXML;

#[cfg(test)]
mod test {
    use quick_xml::se::to_string;
    use serde_xml_rs::from_str;
    use validator::Validate;

    use super::ProjectXML;

    const ORIGINAL_CONTENT: &str = include_str!("test.ScanTailor");

    #[test]
    fn it_deserializes() {
        from_str::<ProjectXML>(ORIGINAL_CONTENT).unwrap();
    }

    #[test]
    fn it_validates() {
        let project: ProjectXML = from_str(ORIGINAL_CONTENT).unwrap();
        project.validate().unwrap();
    }

    #[test]
    fn it_serializes() {
        let project: ProjectXML = from_str(ORIGINAL_CONTENT).unwrap();
        to_string(&project).unwrap();
    }

    #[test]
    fn serialized_output_matches_original() {
        let project: ProjectXML = from_str(ORIGINAL_CONTENT).unwrap();
        let output_content = to_string(&project).unwrap();
        assert_eq!(ORIGINAL_CONTENT, output_content);
    }
}
