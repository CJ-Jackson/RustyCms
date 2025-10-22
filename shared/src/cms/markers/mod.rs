use crate::cms::CmsComponentInfo;

pub trait ComponentDataMarker: Send + Sync {}

impl ComponentDataMarker for Vec<u8> {}

pub trait ComponentInfoMarker: Send + Sync {
    fn component_info() -> CmsComponentInfo;
}
