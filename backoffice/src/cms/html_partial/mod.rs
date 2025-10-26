use crate::cms::data_model::cms_data::ListComponentModel;
use crate::cms::route::CMS_ROUTE;
use crate::common::icon::trash_icon;
use maud::{Markup, html};
use std::sync::Arc;

pub fn positions_partial(
    swap_oob: Option<String>,
    components: Arc<[ListComponentModel]>,
    page_id: u64,
) -> Markup {
    let trash_icon = trash_icon();
    html! {
        div #positions hx-swap-oob=[swap_oob] {
            @if !components.is_empty() {
                form .form hx-patch=(format!("{}/update-position/{}", CMS_ROUTE, page_id)) hx-target="this" hx-swap="outerHTML" {
                    @for component in components.iter() {
                        div .form-group {
                            a .icon .float-right .ml-2 hx-delete=(format!("{}/delete-component/{}/{}", CMS_ROUTE, component.id, page_id)) hx-confirm="Are you sure?"
                                 title="Delete" {
                                (trash_icon)
                            }
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
