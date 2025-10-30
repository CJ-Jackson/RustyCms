use crate::cms::rules::title_rule::TitleRuleExt;
use crate::common::html::context_html::ContextHtmlBuilder;
use crate::common::html::validate::ValidateErrorMessageExt;
use cjtoolkit_structured_validator::common::flag_error::FlagCounter;
use cjtoolkit_structured_validator::types::name::name_alias::{Title, TitleError};
use maud::{Markup, html};
use poem::i18n::Locale;
use serde::{Deserialize, Serialize};
use shared::utils::locale::LocaleExtForResult;
use std::sync::Arc;

#[derive(Deserialize, Default)]
pub struct AddPageForm {
    pub title: String,
}

impl AddPageForm {
    pub async fn as_validated(&self) -> AddPageFormResult {
        AddPageFormResult(
            async {
                let mut flag = FlagCounter::new();

                let title = flag.check(Title::parse_title(Some(self.title.trim())));

                if flag.is_flagged() {
                    return Err(AddPageFormError { title });
                }

                Ok(AddPageFormValidated {
                    title: title.expect("title is not empty"),
                })
            }
            .await,
        )
    }

    pub async fn as_form_html(
        &self,
        context_html_builder: &ContextHtmlBuilder,
        errors: Option<AddPageFormMessage>,
        token: Option<Markup>,
    ) -> Markup {
        let errors = errors.unwrap_or_default();
        let token = token.unwrap_or_default();

        context_html_builder.attach_title("Add Page").attach_content(html! {
            h1 .mt-3 { "Add Page" }
            form hx-boost="true" hx-target="#main-content" .form method="post" {
                (token)
                div .form-group {
                    label .label for="add-title" { "Title" }
                    input .form-item .w-full type="text" #add-title name="title" value=(self.title) required
                        placeholder="Title" {}
                    (errors.title.into_error_html())
                }
                div .form-group {
                    button .btn .btn-sky-blue type="submit" { "Add Page" }
                }
            }
        }).build()
    }
}

pub struct AddPageFormValidated {
    pub title: Title,
}

pub struct AddPageFormError {
    pub title: Result<Title, TitleError>,
}

impl AddPageFormError {
    pub fn as_message(&self, locale: &Locale) -> AddPageFormMessage {
        AddPageFormMessage {
            title: self.title.as_translated_message(locale),
        }
    }
}

pub struct AddPageFormResult(pub Result<AddPageFormValidated, AddPageFormError>);

#[derive(Debug, Clone, Serialize, Default)]
pub struct AddPageFormMessage {
    pub title: Arc<[String]>,
}
