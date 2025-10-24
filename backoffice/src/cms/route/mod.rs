use crate::cms::registry::{registry_ep_create, registry_ep_update_fetch};
use crate::cms::service::cms_page_service::CmsPageService;
use crate::common::html::context_html::ContextHtmlBuilder;
use crate::common::icon::{pencil_square_icon, plus_icon};
use crate::user::pointer::user_pointer::UserPointer;
use crate::user::role::Role;
use maud::{Markup, html};
use poem::web::Path;
use poem::{Route, delete, get, handler, patch};
use shared::context::Dep;
use shared::error::FromErrorStack;

pub mod component;

pub const CMS_ROUTE: &str = "/cms";

#[handler]
async fn cms_list_page(
    Dep(cms_page_service): Dep<CmsPageService>,
    Dep(context_html_builder): Dep<ContextHtmlBuilder>,
    Dep(user_pointer): Dep<UserPointer>,
) -> poem::Result<Markup> {
    let list_page_model = cms_page_service
        .list_page()
        .map_err(poem::Error::from_error_stack)?;
    let edit_icon = pencil_square_icon();
    let add_icon = plus_icon();

    Ok(context_html_builder
        .attach_title("CMS Page List")
        .set_current_tag("id-tag-cms")
        .attach_content(html! {
            h1 { "CMS Page List" }
            table .table-full {
                thead {
                    tr {
                        th { "ID" }
                        th { "Author" }
                        th { "Title" }
                        th { "Added" }
                        th { "Updated" }
                        th { "Status" }
                        th .action { "Action" }
                    }
                }
                tbody {
                    @for page in list_page_model.iter() {
                        tr {
                            td { (page.id) }
                            td { (page.author) }
                            td { (page.title) }
                            td .js-date-local { (page.added.to_rfc3339()) }
                            @if let Some(updated) = &page.updated {
                                td .js-date-local { (updated.to_rfc3339()) }
                            } @else {
                                td { "N/A" }
                            }
                            td { (page.status.as_stringed()) }
                            td .action {
                                @if user_pointer.role == Role::Root || user_pointer.id == page.user_id {
                                    a .icon href=(format!("/cms/amend-page/{}", page.id)) title="Edit"
                                        hx-get=(format!("/cms/amend-page/{}", page.id)) hx-target="#main-content" hx-push-url="true" {
                                        (edit_icon) }
                                }
                            }
                        }
                    }
                }
            }
            div .text-right .mt-3 {
                a .inline-block href="/cms/create-page" title="Create"
                    hx-get="/cms/create-page" hx-target="#main-content" hx-push-url="true" {
                    (add_icon) }
            }
        })
        .build())
}

#[handler]
async fn cms_create_page_get() -> poem::Result<poem::Response> {
    todo!()
}

#[handler]
async fn cms_create_page_post() -> poem::Result<poem::Response> {
    todo!()
}

#[handler]
async fn cms_amend_page_get(Path(page_id): Path<u64>) -> poem::Result<poem::Response> {
    todo!()
}

#[handler]
async fn cms_amend_page_post(Path(page_id): Path<u64>) -> poem::Result<poem::Response> {
    todo!()
}

#[handler]
async fn cms_update_position(Path(page_id): Path<u64>) -> poem::Result<poem::Response> {
    todo!()
}

#[handler]
async fn cms_delete_component(Path(component_id): Path<u64>) -> poem::Result<poem::Response> {
    todo!()
}

pub fn cms_route() -> Route {
    Route::new()
        .at("/list-page", get(cms_list_page))
        .at(
            "/create-page",
            get(cms_create_page_get).post(cms_create_page_post),
        )
        .at(
            "/amend-page/:page_id",
            get(cms_amend_page_get).post(cms_amend_page_post),
        )
        .at("/update-position/:page_id", patch(cms_update_position))
        .at("/create-component", registry_ep_create())
        .at("/component", registry_ep_update_fetch())
        .at(
            "/delete-component/:component_id",
            delete(cms_delete_component),
        )
}
