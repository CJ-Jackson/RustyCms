use crate::cms::form::component_form::markdown_form::MarkdownForm;
use crate::cms::html_partial::positions_partial;
use crate::cms::methods::ComponentMethods;
use crate::cms::query_model::{CreateQuery, UpdateFetchQuery};
use crate::cms::service::cms_page_service::CmsPageService;
use crate::cms::service::component_service::markdown_component_service::MarkdownComponentService;
use maud::{Markup, html};
use poem::http::StatusCode;
use poem::i18n::Locale;
use poem::{IntoResponse, get, handler};
use shared::cms::components::markdown::MarkdownComponent;
use shared::cms::markers::ComponentInfoMarker;
use shared::context::Dep;
use shared::error::FromErrorStack;
use shared::query_string::form::FormQs;
use std::sync::Arc;

#[handler]
async fn markdown_component_create(
    query: CreateQuery,
    Dep(markdown_component_service): Dep<MarkdownComponentService>,
    Dep(cms_page_service): Dep<CmsPageService>,
) -> poem::Result<Markup> {
    let mut form = MarkdownForm::default();
    form.label = "Markdown".to_string();
    form.markdown = "Hello World".to_string();

    let update_fetch_query = markdown_component_service
        .create_component(&query, &form)
        .map_err(poem::Error::from_error_stack)?;

    let list_component_model = cms_page_service
        .list_component(query.page_id as i64)
        .map_err(poem::Error::from_error_stack)?;

    Ok(html! {
        (form.as_form_html(&update_fetch_query, None).await)
        (positions_partial(Some("true".to_string()), Arc::clone(&list_component_model), query.page_id))
    })
}

#[handler]
async fn markdown_component_fetch(
    query: UpdateFetchQuery,
    Dep(markdown_component_service): Dep<MarkdownComponentService>,
) -> poem::Result<Markup> {
    let markdown_component_model = markdown_component_service
        .fetch_component(&query)
        .map_err(poem::Error::from_error_stack)?;

    let mut form = MarkdownForm::default();
    form.label = markdown_component_model.label;
    form.markdown = markdown_component_model.raw_data.content.clone();

    Ok(html! {
        (form.as_form_html(&query, None).await)
    })
}

#[handler]
async fn markdown_component_update(
    query: UpdateFetchQuery,
    Dep(markdown_component_service): Dep<MarkdownComponentService>,
    FormQs(form): FormQs<MarkdownForm>,
    locale: Locale,
) -> poem::Result<poem::Response> {
    let form_validated = form.as_validated().await.0;

    match form_validated {
        Ok(validated) => {
            markdown_component_service
                .update_component(&query, &validated)
                .map_err(poem::Error::from_error_stack)?;
            Ok(html! {
                (form.as_form_html(&query, None).await)
            }
            .into_response())
        }
        Err(verror) => {
            let message = verror.as_message(&locale);
            Ok(html! {
                (form.as_form_html(&query, Some(message)).await)
            }
            .with_status(StatusCode::UNPROCESSABLE_ENTITY)
            .into_response())
        }
    }
}

pub fn markdown_registry_item() -> ComponentMethods {
    ComponentMethods {
        info: MarkdownComponent::component_info(),
        create: get(markdown_component_create),
        update_fetch: get(markdown_component_fetch).patch(markdown_component_update),
    }
}
