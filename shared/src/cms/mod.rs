use std::hash::Hash;

pub mod components;
pub mod markers;
pub mod status;

#[derive(Clone, PartialEq, PartialOrd)]
pub struct CmsComponentInfo {
    pub kind: String,
    pub kind_uuid: String,
}
