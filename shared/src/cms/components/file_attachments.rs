use crate::cms::CmsComponentInfo;
use crate::cms::markers::{ComponentDataMarker, ComponentInfoMarker};

pub struct FileAttachmentsComponent;

impl ComponentDataMarker for FileAttachmentsComponent {
    fn into_data(self) -> Vec<u8> {
        vec![]
    }
}

impl From<Vec<u8>> for FileAttachmentsComponent {
    fn from(_data: Vec<u8>) -> Self {
        Self
    }
}

impl ComponentInfoMarker for FileAttachmentsComponent {
    fn component_info() -> CmsComponentInfo {
        CmsComponentInfo {
            kind: "File Attachments".to_string(),
            kind_uuid: "8a810f99-772c-4f52-91ed-72a4560cf6d0".to_string(),
        }
    }
}
