use crate::cms::query_model::UpdateFetchQuery;
use crate::cms::rules::component_rules::markdown_rules::MarkdownRulesExt;
use crate::cms::rules::title_rule::TitleRuleExt;
use crate::common::html::validate::ValidateErrorMessageExt;
use cjtoolkit_structured_validator::common::flag_error::FlagCounter;
use cjtoolkit_structured_validator::types::description::description_alias::{Text, TextError};
use cjtoolkit_structured_validator::types::name::name_alias::{Field, FieldError};
use maud::{Markup, html};
use poem::i18n::Locale;
use serde::{Deserialize, Serialize};
use shared::utils::locale::LocaleExtForResult;
use std::sync::Arc;

#[derive(Deserialize, Default)]
pub struct MarkdownForm {
    pub label: String,
    pub markdown: String,
}

impl MarkdownForm {
    pub async fn as_validated(&self) -> MarkdownFormResult {
        MarkdownFormResult(
            async {
                let mut flag = FlagCounter::new();

                let label = flag.check(Field::parse_title(Some(self.label.trim())));
                let markdown = flag.check(Text::parse_markdown(Some(self.markdown.trim())));

                if flag.is_flagged() {
                    return Err(MarkdownFormError { label, markdown });
                }

                Ok(MarkdownFormValidated {
                    label: label.expect("label is not empty"),
                    markdown: markdown.expect("summary is not empty"),
                })
            }
            .await,
        )
    }

    pub fn as_form_html(
        &self,
        query: &UpdateFetchQuery,
        errors: Option<MarkdownFormMessage>,
    ) -> Markup {
        let errors = errors.unwrap_or_default();

        html! {
            form .mb-3 .form hx-patch=(query.as_uri()) hx-target="this" hx-ext="alpine-morph" hx-swap="morph" hx-trigger="change" {
                div .form-group {
                    label .label for=(format!("label-group-label-{}", query.id)) { "Label" }
                    input id=(format!("label-group-label-{}", query.id)) .form-item .w-full type="text" name="label" value=(self.label)
                        placeholder="Label" {}
                    span {
                        (errors.label.into_error_html())
                    }
                }
                div .form-group {
                    label .label for=(format!("markdown-group-label-{}", query.id)) { "Markdown" }
                    span data-morph-ignore="true" {
                        textarea x-data=(include_str!("_js/markdown_component.js")) x-model="value" data-value=(self.markdown)
                            id=(format!("markdown-group-label-{}", query.id)) .form-item .w-full name="markdown" {
                            (self.markdown)
                        }
                    }
                    span {
                        (errors.markdown.into_error_html())
                    }
                }
            }
        }
    }
}

pub struct MarkdownFormValidated {
    pub label: Field,
    pub markdown: Text,
}

pub struct MarkdownFormError {
    pub label: Result<Field, FieldError>,
    pub markdown: Result<Text, TextError>,
}

impl MarkdownFormError {
    pub fn as_message(&self, locale: &Locale) -> MarkdownFormMessage {
        MarkdownFormMessage {
            label: self.label.as_translated_message(locale),
            markdown: self.markdown.as_translated_message(locale),
        }
    }
}

pub struct MarkdownFormResult(pub Result<MarkdownFormValidated, MarkdownFormError>);

#[derive(Debug, Clone, Serialize, Default)]
pub struct MarkdownFormMessage {
    pub label: Arc<[String]>,
    pub markdown: Arc<[String]>,
}
