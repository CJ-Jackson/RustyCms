use crate::cms::CmsComponentInfo;
use crate::cms::markers::{ComponentDataMarker, ComponentInfoMarker};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MarkdownComponent {
    pub content: String,
    pub parse_html: String,
}

impl ComponentDataMarker for MarkdownComponent {
    fn into_data(self) -> Vec<u8> {
        rmp_serde::to_vec(&self).expect("failed to serialize markdown")
    }
}

impl From<Vec<u8>> for MarkdownComponent {
    fn from(data: Vec<u8>) -> Self {
        rmp_serde::from_slice(&data).expect("failed to deserialize markdown")
    }
}

impl ComponentInfoMarker for MarkdownComponent {
    fn component_info() -> CmsComponentInfo {
        CmsComponentInfo {
            kind: "Markdown".to_string(),
            kind_uuid: "3191d0cc-0f3f-4a7c-b81a-dcfee22e524b".to_string(),
        }
    }
}
