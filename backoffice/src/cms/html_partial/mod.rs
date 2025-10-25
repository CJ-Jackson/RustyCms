use crate::cms::data_model::cms_data::ListComponentModel;
use crate::cms::route::CMS_ROUTE;
use maud::{Markup, html};
use std::sync::Arc;

pub fn positions_partial(
    swap_oob: Option<String>,
    components: Arc<[ListComponentModel]>,
    page_id: u64,
) -> Markup {
    html! {
        div #positions hx-swap-oob=[swap_oob] {
            @if !components.is_empty() {
                form .form hx-patch=(format!("{}/update-position/{}", CMS_ROUTE, page_id)) hx-target="this" hx-swap="outerHTML" {
                    @for component in components.iter() {
                        div .form-group {
                            label .label for=(format!("component-position-{}", component.id)) { (component.label) }
                            input .form-item .w-full type="number" id=(format!("component-position-{}", component.id))
                                name=(format!("mapping[{}]", component.id)) value=(component.position) {}
                        }
                    }
                    div .form-group mt-3 {
                        button .btn .btn-sky-blue type="submit" { "Update Positions" }
                    }
                }
            }
        }
    }
}

pub fn component_partial(
    swap_oob: Option<String>,
    components: Arc<[ListComponentModel]>,
) -> Markup {
    html! {
        div #components hx-swap-oob=[swap_oob] {
            @if !components.is_empty() {
                @for component in components.iter() {
                    span hx-get=(component.as_query().as_uri()) hx-trigger="load" hx-swap="outerHTML" {}
                }
            }
        }
    }
}
