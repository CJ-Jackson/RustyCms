use poem::RouteMethod;
use shared::cms::CmsComponentInfo;

pub struct ComponentMethods {
    pub info: CmsComponentInfo,
    pub create: RouteMethod,
    pub update_fetch: RouteMethod,
}
