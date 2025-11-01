use crate::cms::extractor::HeaderId;
use crate::cms::form::component_form::common_label_form::CommonLabelForm;
use crate::cms::html_partial::file_attachments_html_partial::{
    file_attachments_form_field_partial, file_attachments_form_partial,
    file_attachments_list_partial,
};
use crate::cms::html_partial::positions_partial;
use crate::cms::methods::ComponentMethods;
use crate::cms::query_model::{CreateQuery, UpdateFetchQuery};
use crate::cms::service::cms_attachment_service::CmsAttachmentService;
use crate::cms::service::cms_page_service::CmsPageService;
use crate::cms::service::component_service::common_label_service::CommonLabelService;
use crate::common::html::flash_partial::flash_partial;
use maud::{Markup, html};
use poem::i18n::Locale;
use poem::web::Multipart;
use poem::{IntoResponse, get, handler};
use shared::cms::components::file_attachments::FileAttachmentsComponent;
use shared::cms::markers::ComponentInfoMarker;
use shared::utils::context::Dep;
use shared::utils::error::FromErrorStack;
use shared::utils::flash::Flash;
use shared::utils::query_string::form::FormQs;
use shared::utils::route_header::route_header;
use std::sync::Arc;

#[handler]
async fn file_attachments_component_create(
    query: CreateQuery,
    Dep(common_label_service): Dep<CommonLabelService>,
    Dep(cms_page_service): Dep<CmsPageService>,
) -> poem::Result<Markup> {
    let mut form = CommonLabelForm::default();
    form.label = "File Attachments".to_string();

    let update_fetch_query = common_label_service
        .create_component(&query, &form)
        .map_err(poem::Error::from_error_stack)?;

    let list_component_model = cms_page_service
        .list_component(query.page_id as i64)
        .map_err(poem::Error::from_error_stack)?;

    Ok(html! {
        h4 .mt-3 { "File Attachments" }
        (form.as_form_html(&update_fetch_query, None))
        (file_attachments_list_partial(None, &update_fetch_query, vec![].into()))
        (positions_partial(Some("true".to_string()), Arc::clone(&list_component_model), query.page_id))
    })
}

#[handler]
async fn file_attachments_component_fetch(
    query: UpdateFetchQuery,
    Dep(common_label_service): Dep<CommonLabelService>,
    Dep(cms_attachment_service): Dep<CmsAttachmentService>,
) -> poem::Result<Markup> {
    let common_label_model = common_label_service
        .fetch_component(&query)
        .map_err(poem::Error::from_error_stack)?;

    let mut form = CommonLabelForm::default();
    form.label = common_label_model.label;

    let file_list = cms_attachment_service
        .list_file_by_component_id(query.id as i64)
        .map_err(poem::Error::from_error_stack)?;

    Ok(html! {
        h4 .mt-3 { "File Attachments" }
        (form.as_form_html(&query, None))
        (file_attachments_list_partial(None, &query, file_list))
    })
}

#[handler]
async fn file_attachments_component_fetch_form(query: UpdateFetchQuery) -> Markup {
    html! {
        (file_attachments_form_partial(&query))
    }
}

#[handler]
async fn file_attachments_component_fetch_form_field() -> Markup {
    html! {
        (file_attachments_form_field_partial())
    }
}

#[handler]
async fn file_attachments_component_update(
    query: UpdateFetchQuery,
    Dep(common_label_service): Dep<CommonLabelService>,
    FormQs(form): FormQs<CommonLabelForm>,
    locale: Locale,
) -> poem::Result<poem::Response> {
    let form_validated = form.as_validated().await.0;

    match form_validated {
        Ok(validated) => {
            common_label_service
                .update_component(&query, &validated)
                .map_err(poem::Error::from_error_stack)?;
            Ok(html! {
                (form.as_form_html(&query, None))
                span id=(format!{"component-position-label-{}", query.id}) hx-swap-oob="true"
                    { (validated.label.as_str()) }
            }
            .into_response())
        }
        Err(verror) => {
            let message = verror.as_message(&locale);
            Ok(html! {
                (form.as_form_html(&query, Some(message)))
            }
            .with_status(poem::http::StatusCode::UNPROCESSABLE_ENTITY)
            .into_response())
        }
    }
}

#[handler]
async fn file_attachments_component_upload(
    query: UpdateFetchQuery,
    Dep(cms_attachment_service): Dep<CmsAttachmentService>,
    mut multipart: Multipart,
) -> poem::Result<Markup> {
    while let Some(file) = multipart.next_field().await? {
        if file.name().unwrap_or_default() == "file" {
            cms_attachment_service
                .add_file(query.id as i64, file)
                .await
                .map_err(poem::Error::from_error_stack)?;
        }
    }

    let file_list = cms_attachment_service
        .list_file_by_component_id(query.id as i64)
        .map_err(poem::Error::from_error_stack)?;

    Ok(html! {
        (file_attachments_list_partial(None, &query, file_list))
        (flash_partial(Flash::Success {
            msg: "File uploaded successfully".to_string(),
        }))
    })
}

#[handler]
async fn file_attachments_component_delete(
    query: UpdateFetchQuery,
    Dep(cms_attachment_service): Dep<CmsAttachmentService>,
    HeaderId(id): HeaderId,
) -> poem::Result<Markup> {
    cms_attachment_service
        .delete_file_by_id(id as i64)
        .map_err(poem::Error::from_error_stack)?;

    let file_list = cms_attachment_service
        .list_file_by_component_id(query.id as i64)
        .map_err(poem::Error::from_error_stack)?;

    Ok(html! {
        (file_attachments_list_partial(None, &query, file_list))
        (flash_partial(Flash::Success {
            msg: "File deleted successfully".to_string(),
        }))
    })
}

pub fn file_attachments_registry_item() -> ComponentMethods {
    ComponentMethods {
        info: FileAttachmentsComponent::component_info(),
        create: get(file_attachments_component_create),
        update_fetch: get(route_header("X-Route", file_attachments_component_fetch)
            .at("form", file_attachments_component_fetch_form)
            .at("form-field", file_attachments_component_fetch_form_field))
        .patch(file_attachments_component_update)
        .post(file_attachments_component_upload)
        .delete(file_attachments_component_delete),
    }
}
