use crate::cms::data_model::cms_data::ListFileAttachmentModel;
use crate::cms::query_model::UpdateFetchQuery;
use crate::common::icon::{plus_icon, trash_icon};
use maud::{Markup, html};
use serde_json::json;
use std::sync::Arc;

pub fn file_attachments_list_partial(
    swap_oob: Option<&str>,
    update_fetch_query: &UpdateFetchQuery,
    file_list: Arc<[ListFileAttachmentModel]>,
) -> Markup {
    html! {
        div id=(format!("file-attachment-list-{}", update_fetch_query.id)) hx-swap-oob=[swap_oob] {
            @if file_list.is_empty() {
                p { "No file attached" }
            } @else {
                @for file in file_list.iter() {
                    div .file-attachment {
                        a .file-attachment-link href=(file.file_path.clone()) target="_blank" {
                            (file.file_name.clone())
                        }
                        a .file-attachment-delete hx-delete=(update_fetch_query.as_uri()) hx-confirm="Are you sure?"
                            title="Delete" hx-headers=(json!({"X-Delete-Id": file.id}))
                            hx-target=(format!("#file-attachment-list-{}", update_fetch_query.id)) { (trash_icon()) }
                    }
                }
            }
            span hx-get=(update_fetch_query.as_uri()) hx-headers=(json!({"X-Route": "form"})) hx-trigger="load" {}
        }
    }
}

pub fn file_attachments_form_partial(update_fetch_query: &UpdateFetchQuery) -> Markup {
    html! {
        h5 { "Upload Files" }
        form .form hx-post=(update_fetch_query.as_uri()) hx-target=(format!("#file-attachment-list-{}", update_fetch_query.id))
        hx-swap="outerHTML" hx-encoding="multipart/form-data" {
            div id=(format!("file-attachment-form-{}", update_fetch_query.id)) {
                (file_attachments_form_field_partial())
            }
            a .icon hx-get=(update_fetch_query.as_uri()) hx-headers=(json!({"X-Route": "form-field"}))
            hx-target=((format!("#file-attachment-form-{}", update_fetch_query.id))) hx-swap="beforeend" title="Add File Field" { (plus_icon()) }
            button .btn .btn-sky-blue type="submit" { "Upload Files" }
        }
    }
}

pub fn file_attachments_form_field_partial() -> Markup {
    html! {
        input type="file" name="file" {}
    }
}
