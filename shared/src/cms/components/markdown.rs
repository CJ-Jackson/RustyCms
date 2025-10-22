use crate::cms::CmsComponentInfo;
use crate::cms::markers::{ComponentDataMarker, ComponentInfoMarker};

pub struct Markdown {
    pub content: String,
    pub parse_html: String,
}

impl ComponentDataMarker for Markdown {}

impl ComponentInfoMarker for Markdown {
    fn component_info() -> CmsComponentInfo {
        CmsComponentInfo {
            kind: "Markdown".to_string(),
            kind_uuid: "3191d0cc-0f3f-4a7c-b81a-dcfee22e524b".to_string(),
        }
    }
}
