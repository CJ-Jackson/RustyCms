use crate::cms::methods::ComponentMethods;
use maud::Markup;
use poem::{get, handler};
use shared::cms::components::markdown::MarkdownComponent;
use shared::cms::markers::ComponentInfoMarker;

#[handler]
fn markdown_component_create() -> poem::Result<Markup> {
    todo!()
}

#[handler]
fn markdown_component_fetch() -> poem::Result<Markup> {
    todo!()
}

#[handler]
fn markdown_component_update() -> poem::Result<Markup> {
    todo!()
}

pub fn markdown_registry_item() -> ComponentMethods {
    ComponentMethods {
        info: MarkdownComponent::component_info(),
        create: get(markdown_component_create),
        update_fetch: get(markdown_component_fetch).patch(markdown_component_update),
    }
}
