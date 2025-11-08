use crate::cms::rules::summary_rule::SummaryRuleExt;
use crate::cms::rules::title_rule::TitleRuleExt;
use crate::common::html::validate::ValidateErrorMessageExt;
use cjtoolkit_structured_validator::common::flag_error::FlagCounter;
use cjtoolkit_structured_validator::types::description::description_alias::{
    Summary, SummaryError,
};
use cjtoolkit_structured_validator::types::name::name_alias::{Title, TitleError};
use maud::{Markup, html};
use poem::i18n::Locale;
use serde::{Deserialize, Serialize};
use shared::cms::status::CmsPageStatus;
use shared::utils::locale::LocaleExtForResult;
use std::sync::Arc;

#[derive(Deserialize, Default)]
pub struct AmendPageForm {
    pub title: String,
    pub summary: String,
    pub status: CmsPageStatus,
}

impl AmendPageForm {
    pub async fn as_validated(&self) -> AmendPageFormResult {
        AmendPageFormResult(
            async {
                let mut flag = FlagCounter::new();

                let title = flag.check(Title::parse_title(Some(self.title.trim())));
                let summary = flag.check(Summary::parse_summary(Some(self.summary.trim())));

                if flag.is_flagged() {
                    return Err(AmendPageFormError { title, summary });
                }

                Ok(AmendPageFormValidated {
                    title: title.expect("title is not empty"),
                    summary: summary.expect("summary is not empty"),
                    status: self.status,
                })
            }
            .await,
        )
    }

    pub async fn as_form_html(&self, errors: Option<AmendPageFormMessage>) -> Markup {
        let errors = errors.unwrap_or_default();

        html! {
            form .form hx-boost="true" hx-target="this" method="post" hx-swap="outerHTML" {
                div .form-group {
                    label .label for="amend-title" { "Title" }
                    input .form-item .w-full type="text" #amend-title name="title" value=(self.title) required
                        placeholder="Title" {}
                    (errors.title.into_error_html())
                }
                div .form-group {
                    label .label for="amend-summary" { "Summary" }
                    textarea .form-item .w-full #amend-summary name="summary" required {
                        (self.summary)
                    }
                    (errors.summary.into_error_html())
                }
                div .form-group {
                    label .label for="amend-status" { "Status" }
                    select .form-item .w-full #amend-status name="status" required {
                        (self.status.html_option())
                    }
                }
                div .form-group {
                    button .btn .btn-sky-blue .cursor-pointer type="submit" { "Save Info and Status" }
                }
            }
        }
    }
}

pub struct AmendPageFormValidated {
    pub title: Title,
    pub summary: Summary,
    pub status: CmsPageStatus,
}

pub struct AmendPageFormError {
    pub title: Result<Title, TitleError>,
    pub summary: Result<Summary, SummaryError>,
}

impl AmendPageFormError {
    pub fn as_message(&self, locale: &Locale) -> AmendPageFormMessage {
        AmendPageFormMessage {
            title: self.title.as_translated_message(locale),
            summary: self.summary.as_translated_message(locale),
        }
    }
}

pub struct AmendPageFormResult(pub Result<AmendPageFormValidated, AmendPageFormError>);

#[derive(Debug, Clone, Serialize, Default)]
pub struct AmendPageFormMessage {
    pub title: Arc<[String]>,
    pub summary: Arc<[String]>,
}
