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
                form hx-patch=(format!("{}/update-positon/{}", CMS_ROUTE, page_id)) hx-target="this" hx-swap="outerHTML" {
                    @for component in components.iter() {
                        div .form-group {
                            label .label for=(format!("component-position-{}", component.id)) { (component.label) }
                            input type="number" id=(format!("component-position-{}", component.id))
                                name=(format!("position[{}]", component.id)) value=(component.position) {}
                        }
                    }
                    div .form-group {
                        button .btn .btn-sky-blue type="submit" { "Update Positions" }
                    }
                }
            }
        }
    }
}
