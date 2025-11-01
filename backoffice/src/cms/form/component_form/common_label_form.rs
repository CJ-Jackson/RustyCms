use crate::cms::query_model::UpdateFetchQuery;
use crate::cms::rules::title_rule::TitleRuleExt;
use crate::common::html::validate::ValidateErrorMessageExt;
use cjtoolkit_structured_validator::common::flag_error::FlagCounter;
use cjtoolkit_structured_validator::types::name::name_alias::{Field, FieldError};
use maud::{Markup, html};
use poem::i18n::Locale;
use serde::{Deserialize, Serialize};
use shared::utils::locale::LocaleExtForResult;
use std::sync::Arc;

#[derive(Deserialize, Default)]
pub struct CommonLabelForm {
    pub label: String,
}

impl CommonLabelForm {
    pub async fn as_validated(&self) -> CommonFormResult {
        CommonFormResult(
            async {
                let mut flag = FlagCounter::new();

                let label = flag.check(Field::parse_title(Some(self.label.trim())));

                if flag.is_flagged() {
                    return Err(CommonFormError { label });
                }

                Ok(CommonFormValidated {
                    label: label.expect("label is not empty"),
                })
            }
            .await,
        )
    }

    pub fn as_form_html(
        &self,
        query: &UpdateFetchQuery,
        errors: Option<CommonFormMessage>,
    ) -> Markup {
        let errors = errors.unwrap_or_default();

        html! {
            form .mb-3 .form hx-patch=(query.as_uri()) hx-swap="outerHTML" hx-trigger="change" {
                div .form-group id=(format!("label-group-{}", query.id)) {
                    label .label for=(format!("label-group-label-{}", query.id)) { "Label" }
                    input id=(format!("label-group-label-{}", query.id)) .form-item .w-full type="text" name="label" value=(self.label)
                        placeholder="Label" {}
                    (errors.label.into_error_html())
                }
            }
        }
    }
}

pub struct CommonFormValidated {
    pub label: Field,
}

pub struct CommonFormError {
    pub label: Result<Field, FieldError>,
}

impl CommonFormError {
    pub fn as_message(&self, locale: &Locale) -> CommonFormMessage {
        CommonFormMessage {
            label: self.label.as_translated_message(locale),
        }
    }
}

pub struct CommonFormResult(pub Result<CommonFormValidated, CommonFormError>);

#[derive(Debug, Clone, Serialize, Default)]
pub struct CommonFormMessage {
    pub label: Arc<[String]>,
}
