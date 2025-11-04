use crate::FILES_ROUTE;
use crate::cms::data_model::cms_data::ListFileAttachmentModel;
use crate::cms::query_model::UpdateFetchQuery;
use crate::common::icon::{arrow_path_icon, link_icon, minus_icon, plus_icon, trash_icon};
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
                div .file-attachment-grid {
                    @for file in file_list.iter() {
                        div .file-attachment {
                            div .label { "File Name:" }
                            span title=(file.file_name.clone()) { (file.file_name.clone()) }
                            div .label .mt-1 { "File Type:" }
                            (file.file_type.clone())
                            div .label .mt-1 { "Uploaded At:" }
                            span x-init="await formatToLocalTime($el)" { (file.uploaded.to_rfc3339()) }

                            div .text-right .mt-2 {
                                a .file-attachment-delete .icon hx-delete=(update_fetch_query.as_uri()) hx-confirm="Are you sure?"
                                title="Delete" hx-headers=(json!({"X-Delete-Id": file.id}))
                                hx-target=(format!("#file-attachment-list-{}", update_fetch_query.id)) { (trash_icon) }
                                a .file-attachment-link .icon href=(format!("{}{}", FILES_ROUTE, file.file_path.clone())) title="Link to file" target="_blank" {
                                    (link_icon)
                                }
                            }
                        }
                    }
                }
            }
            span hx-get=(update_fetch_query.as_uri()) hx-headers=(json!({"X-Route": "form"})) hx-trigger="load" hx-swap="outerHTML" {}
        }
    }
}

pub fn file_attachments_form_partial(update_fetch_query: &UpdateFetchQuery) -> Markup {
    let js_data = include_str!("_js/file_attachment_form_data.js");
    html! {
        div x-cloak x-data=(js_data) {
            div .flex {
                h5 class="basis-1/2" { "Upload Files" }
                div class="basis-1/2 text-right" {
                    a class="inline-block ml-2 size-5! cursor-pointer" x-show="show" x-on:click="reset()" title="Reset" { (arrow_path_icon()) }
                    a class="inline-block ml-2 size-5! cursor-pointer" x-show="show" x-on:click="remove()" title="Remove File Field" { (minus_icon()) }
                    a class="inline-block ml-2 size-5! cursor-pointer" x-on:click="add()" title="Add File Field" { (plus_icon()) }
                }
            }
            form .form hx-post=(update_fetch_query.as_uri()) hx-target=(format!("#file-attachment-list-{}", update_fetch_query.id))
                hx-swap="outerHTML" hx-encoding="multipart/form-data" {
                div .upload-form {
                    template x-for="i in count" {
                        (file_attachments_form_field_partial())
                    }
                }

                button .btn .btn-sky-blue .mt-3 type="submit" { "Upload Files" }
            }
        }
    }
}

pub fn file_attachments_form_field_partial() -> Markup {
    html! {
        input .upload-form-file type="file" name="file" {}
    }
}
