use crate::cms::CmsComponentInfo;

pub trait ComponentDataMarker: Send + Sync {
    fn into_data(self) -> Vec<u8>;
}

impl ComponentDataMarker for Vec<u8> {
    fn into_data(self) -> Vec<u8> {
        self
    }
}

pub trait ComponentInfoMarker: Send + Sync {
    fn component_info() -> CmsComponentInfo;
}
