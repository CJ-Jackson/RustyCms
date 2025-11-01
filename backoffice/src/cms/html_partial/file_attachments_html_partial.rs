use crate::FILES_ROUTE;
use crate::cms::data_model::cms_data::ListFileAttachmentModel;
use crate::cms::query_model::UpdateFetchQuery;
use crate::common::icon::{link_icon, plus_icon, trash_icon};
use maud::{Markup, html};
use serde_json::json;
use std::sync::Arc;

pub fn file_attachments_list_partial(
    swap_oob: Option<&str>,
    update_fetch_query: &UpdateFetchQuery,
    file_list: Arc<[ListFileAttachmentModel]>,
) -> Markup {
    let trash_icon = trash_icon();
    let link_icon = link_icon();
    html! {
        div id=(format!("file-attachment-list-{}", update_fetch_query.id)) hx-swap-oob=[swap_oob] {
            @if file_list.is_empty() {
                p { "No file attached" }
            } @else {
                div .file-attachment-flex {
                    @for file in file_list.iter() {
                        div .file-attachment {
                            div .label { "File Name:" }
                            (file.file_name.clone())
                            div .label .mt-1 { "File Type:" }
                            (file.file_type.clone())
                            div .label .mt-1 { "Uploaded At:" }
                            span .js-date-local { (file.uploaded.to_rfc3339()) }

                            div .text-right .mt-2 {
                                a .file-attachment-delete .icon hx-delete=(update_fetch_query.as_uri()) hx-confirm="Are you sure?"
                                title="Delete" hx-headers=(json!({"X-Delete-Id": file.id}))
                                hx-target=(format!("#file-attachment-list-{}", update_fetch_query.id)) { (trash_icon) }
                                a .file-attachment-link .icon href=(format!("{}{}", FILES_ROUTE, file.file_path.clone())) target="_blank" {
                                    (link_icon)
                                }
                            }
                        }
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
            div .upload-form id=(format!("file-attachment-form-{}", update_fetch_query.id)) {
                (file_attachments_form_field_partial())
            }
            div .text-right {
                a class="inline-block ml-2 size-5! cursor-pointer" hx-get=(update_fetch_query.as_uri()) hx-headers=(json!({"X-Route": "form-field"}))
                hx-target=((format!("#file-attachment-form-{}", update_fetch_query.id))) hx-swap="beforeend" title="Add File Field" { (plus_icon()) }
            }
            button .btn .btn-sky-blue .mt-3 type="submit" { "Upload Files" }
        }
    }
}

pub fn file_attachments_form_field_partial() -> Markup {
    html! {
        input .upload-form-file type="file" name="file" {}
    }
}
