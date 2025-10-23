use crate::context::{Context, ContextError, FromContext};
use error_stack::Report;

pub mod components;
pub mod markers;
pub mod status;

#[derive(Clone, PartialEq, PartialOrd)]
pub struct CmsComponentInfo {
    pub kind: String,
    pub kind_uuid: String,
}

impl FromContext for CmsComponentInfo {
    async fn from_context(ctx: &'_ Context<'_>) -> Result<Self, Report<ContextError>> {
        let req = ctx.req_result()?;
        let cms_component_info = req
            .data::<Self>()
            .ok_or_else(|| Report::new(ContextError::Other))?;
        Ok(cms_component_info.clone())
    }
}
