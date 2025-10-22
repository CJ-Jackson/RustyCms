use crate::cms::methods::ComponentMethods;
use poem::{get, handler};
use shared::cms::components::markdown::MarkdownComponent;
use shared::cms::markers::ComponentInfoMarker;

#[handler]
fn markdown_component_create() -> poem::Result<poem::Response> {
    todo!()
}

#[handler]
fn markdown_component_fetch() -> poem::Result<poem::Response> {
    todo!()
}

#[handler]
fn markdown_component_update() -> poem::Result<poem::Response> {
    todo!()
}

pub fn markdown_registry_item() -> ComponentMethods {
    ComponentMethods {
        info: MarkdownComponent::component_info(),
        create: get(markdown_component_create),
        update_fetch: get(markdown_component_fetch).patch(markdown_component_update),
    }
}
