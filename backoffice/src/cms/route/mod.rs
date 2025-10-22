use crate::cms::registry::{registry_ep_create, registry_ep_update_fetch};
use poem::Route;

pub mod component;

pub const CMS_ROUTE: &str = "/cms";

pub fn cms_route() -> Route {
    Route::new()
        .at("/create-component", registry_ep_create())
        .at("/component", registry_ep_update_fetch())
}
