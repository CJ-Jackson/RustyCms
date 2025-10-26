use crate::cms::form::add_page_form::AddPageForm;
use crate::cms::form::amend_page_form::AmendPageForm;
use crate::cms::form::component_position_form::ComponentPositionForm;
use crate::cms::html_partial::{component_partial, positions_partial};
use crate::cms::query_model::CreateQueryExt;
use crate::cms::registry::{registry_ep_create, registry_ep_update_fetch, registry_item};
use crate::cms::service::cms_page_service::CmsPageService;
use crate::cms::service::cms_permission_check_service::CmsPermissionCheckService;
use crate::common::html::context_html::ContextHtmlBuilder;
use crate::common::html::flash_partial::flash_partial;
use crate::common::icon::{pencil_square_icon, plus_icon};
use crate::user::pointer::user_pointer::UserPointer;
use crate::user::role::Role;
use maud::{Markup, html};
use poem::http::StatusCode;
use poem::i18n::Locale;
use poem::session::Session;
use poem::web::{CsrfToken, CsrfVerifier, Path, Redirect};
use poem::{IntoResponse, Route, delete, get, handler, patch};
use serde_qs::Config;
use shared::context::Dep;
use shared::csrf::{CsrfTokenHtml, CsrfVerifierError};
use shared::error::{ExtraResultExt, FromErrorStack};
use shared::flash::{Flash, FlashMessage};
use shared::htmx::HtmxHeader;
use shared::query_string::form::FormQs;
use shared::query_string::serde_qs_config::with_serde_qs_config;
use std::sync::Arc;

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
                            @if let Some(updated) = page.updated.as_ref() {
                                td .js-date-local { (updated.to_rfc3339()) }
                            } @else {
                                td { "N/A" }
                            }
                            td { (page.status.as_stringed()) }
                            td .action {
                                @if user_pointer.role == Role::Root || user_pointer.id == page.user_id {
                                    a .icon href=(format!("{}/amend-page/{}", CMS_ROUTE, page.id)) title="Edit"
                                        hx-get=(format!("{}/amend-page/{}", CMS_ROUTE, page.id)) hx-target="#main-content" hx-push-url="true" {
                                        (edit_icon) }
                                }
                            }
                        }
                    }
                }
            }
            div .text-right .mt-3 {
                a .inline-block href=(format!("{}/create-page", CMS_ROUTE)) title="Create"
                    hx-get=(format!("{}/create-page", CMS_ROUTE)) hx-target="#main-content" hx-push-url="true" {
                    (add_icon) }
            }
        })
        .build())
}

#[handler]
async fn cms_create_page_get(
    Dep(context_html_builder): Dep<ContextHtmlBuilder>,
    csrf_token: &CsrfToken,
) -> poem::Result<Markup> {
    let add_page_form = AddPageForm::default();
    Ok(add_page_form
        .as_form_html(&context_html_builder, None, Some(csrf_token.as_html()))
        .await)
}

#[handler]
async fn cms_create_page_post(
    Dep(context_html_builder): Dep<ContextHtmlBuilder>,
    Dep(cms_page_service): Dep<CmsPageService>,
    FormQs(add_page_form): FormQs<AddPageForm>,
    csrf_token: &CsrfToken,
    csrf_verifier: &CsrfVerifier,
    session: &Session,
    htmx_header: HtmxHeader,
) -> poem::Result<poem::Response> {
    csrf_verifier
        .verify(add_page_form.csrf_token.as_str())
        .map_err(poem::Error::from_error_stack)?;
    let validated_form = add_page_form.as_validated().await.0;
    let l = &context_html_builder.locale;
    match validated_form {
        Ok(validated) => {
            let returning_id = cms_page_service
                .add_page(&validated)
                .log_it()
                .map_err(poem::Error::from_error_stack)?;
            session.flash(Flash::Success {
                msg: format!("Successfully create page {}", returning_id.0),
            });
            Ok(htmx_header.do_location(
                Redirect::see_other(format!("{}/amend-page/{}", CMS_ROUTE, returning_id.0)),
                "#main-content",
            ))
        }
        Err(error) => {
            let error_message = error.as_message(l);
            context_html_builder.attach_form_flash_error();
            Ok(add_page_form
                .as_form_html(
                    &context_html_builder,
                    Some(error_message),
                    Some(csrf_token.as_html()),
                )
                .await
                .with_status(StatusCode::UNPROCESSABLE_ENTITY)
                .into_response())
        }
    }
}

#[handler]
async fn cms_amend_page_get(
    Path(page_id): Path<u64>,
    Dep(context_html_builder): Dep<ContextHtmlBuilder>,
    Dep(cms_page_service): Dep<CmsPageService>,
    Dep(cms_permission_check_service): Dep<CmsPermissionCheckService>,
) -> poem::Result<Markup> {
    cms_permission_check_service
        .check_permission_by_page_id(page_id as i64)
        .map_err(poem::Error::from_error_stack)?;

    let page_model = cms_page_service
        .fetch_page(page_id as i64)
        .map_err(poem::Error::from_error_stack)?;

    let mut amend_page_form = AmendPageForm::default();
    amend_page_form.title = page_model.title.clone();
    amend_page_form.summary = page_model.summary;
    amend_page_form.status = page_model.status;

    let title = format!("CMS Page {}", page_model.title);

    let list_component_model = cms_page_service
        .list_component(page_id as i64)
        .map_err(poem::Error::from_error_stack)?;

    Ok(context_html_builder
        .attach_title(&title)
        .attach_content(html! {
            h1 { (title) }
            (amend_page_form.as_form_html(None).await)
            div .flex .flex-row .mt-10 {
                div class="basis-4/5 pr-6" {
                    h3 { "Components" }
                    span { "All components are auto-save" }
                    (component_partial(None, Arc::clone(&list_component_model)))
                    h3 .mt-5 { "Positions" }
                    (positions_partial(None, Arc::clone(&list_component_model), page_id))
                }
                div class="basis-1/5" {
                    h3 { "Add Component" }
                    @for item in registry_item().iter() {
                        span .btn .btn-sky-blue hx-get=(item.as_create_query(page_id).as_uri())
                        hx-target="#components" hx-swap="beforeend" { (item.kind) }
                    }
                }
            }

        })
        .build())
}

#[handler]
async fn cms_amend_page_post(
    Path(page_id): Path<u64>,
    Dep(cms_page_service): Dep<CmsPageService>,
    Dep(cms_permission_check_service): Dep<CmsPermissionCheckService>,
    FormQs(amend_page_form): FormQs<AmendPageForm>,
    locale: Locale,
) -> poem::Result<poem::Response> {
    cms_permission_check_service
        .check_permission_by_page_id(page_id as i64)
        .map_err(poem::Error::from_error_stack)?;

    let validated_form = amend_page_form.as_validated().await.0;
    match validated_form {
        Ok(validated) => {
            cms_page_service
                .update_page(page_id as i64, &validated)
                .map_err(poem::Error::from_error_stack)?;
            Ok(html! {
                (amend_page_form.as_form_html(None).await)
                (flash_partial(Flash::Success {
                    msg: "Updated Info and Status".to_string()
                }))
            }
            .into_response())
        }
        Err(error) => {
            let error_message = error.as_message(&locale);
            Ok(html! {
                (amend_page_form.as_form_html(Some(error_message)).await)
                (flash_partial(Flash::Error {
                    msg: "Failed to update info and status".to_string()
                }))
            }
            .with_status(StatusCode::UNPROCESSABLE_ENTITY)
            .into_response())
        }
    }
}

#[handler]
async fn cms_update_position(
    Path(page_id): Path<u64>,
    Dep(cms_page_service): Dep<CmsPageService>,
    Dep(cms_permission_check_service): Dep<CmsPermissionCheckService>,
    FormQs(component_position_form): FormQs<ComponentPositionForm>,
) -> poem::Result<Markup> {
    cms_permission_check_service
        .check_permission_by_page_id(page_id as i64)
        .map_err(poem::Error::from_error_stack)?;

    cms_page_service
        .update_component_position(&component_position_form)
        .map_err(poem::Error::from_error_stack)?;

    let list_component_model = cms_page_service
        .list_component(page_id as i64)
        .map_err(poem::Error::from_error_stack)?;

    Ok(html! {
        (positions_partial(Some("true".to_string()), Arc::clone(&list_component_model), page_id))
        (component_partial(Some("true".to_string()), Arc::clone(&list_component_model)))
        (flash_partial(Flash::Success {
            msg: "Updated Positions".to_string()
        }))
    })
}

#[handler]
async fn cms_delete_component(
    Path((component_id, page_id)): Path<(u64, u64)>,
    Dep(cms_page_service): Dep<CmsPageService>,
    Dep(cms_permission_check_service): Dep<CmsPermissionCheckService>,
) -> poem::Result<Markup> {
    cms_permission_check_service
        .check_permission_by_component_id(component_id as i64)
        .map_err(poem::Error::from_error_stack)?;

    cms_page_service
        .delete_component(component_id as i64)
        .map_err(poem::Error::from_error_stack)?;

    let list_component_model = cms_page_service
        .list_component(page_id as i64)
        .map_err(poem::Error::from_error_stack)?;

    Ok(html! {
        (positions_partial(Some("true".to_string()), Arc::clone(&list_component_model), page_id))
        (component_partial(Some("true".to_string()), Arc::clone(&list_component_model)))
        (flash_partial(Flash::Success {
            msg: "Deleted component successfully".to_string()
        }))
    })
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
        .at(
            "/update-position/:page_id",
            patch(with_serde_qs_config(
                Config::default().use_form_encoding(true),
                cms_update_position,
            )),
        )
        .at("/create-component", registry_ep_create())
        .at("/component", registry_ep_update_fetch())
        .at(
            "/delete-component/:component_id/:page_id",
            delete(cms_delete_component),
        )
}
