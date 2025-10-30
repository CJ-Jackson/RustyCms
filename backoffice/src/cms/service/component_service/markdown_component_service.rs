use crate::cms::data_model::cms_data::{
    CreateComponentModel, FetchComponentModel, UpdateComponentModel,
};
use crate::cms::form::component_form::markdown_form::{MarkdownForm, MarkdownFormValidated};
use crate::cms::query_model::{CreateQuery, UpdateFetchQuery};
use crate::cms::repository::cms_repository::CmsRepository;
use error_stack::{Report, ResultExt};
use shared::cms::CmsComponentInfo;
use shared::cms::components::markdown::MarkdownComponent;
use shared::cms::markers::ComponentDataMarker;
use shared::utils::context::{Context, ContextError, FromContext};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MarkdownComponentServiceError {
    #[error("db error")]
    DbError,
    #[error("not found error")]
    NotFound,
}

pub struct MarkdownComponentService {
    cms_repository: CmsRepository,
    pub cms_component_info: CmsComponentInfo,
}

impl MarkdownComponentService {
    pub fn new(cms_repository: CmsRepository, cms_component_info: CmsComponentInfo) -> Self {
        Self {
            cms_repository,
            cms_component_info,
        }
    }

    pub fn create_component(
        &self,
        query: &CreateQuery,
        markdown_form: &MarkdownForm,
    ) -> Result<UpdateFetchQuery, Report<MarkdownComponentServiceError>> {
        let create_component_model = CreateComponentModel {
            page_id: query.page_id as i64,
            kind_uuid: self.cms_component_info.kind_uuid.clone(),
            raw_data: MarkdownComponent {
                content: markdown_form.markdown.clone(),
                parse_html: markdown_form.markdown.clone(),
            }
            .into_data(),
            label: markdown_form.label.clone(),
        };

        let returning_id = self
            .cms_repository
            .create_component(create_component_model)
            .change_context(MarkdownComponentServiceError::DbError)?;

        Ok(UpdateFetchQuery::new(
            self.cms_component_info.kind_uuid.clone(),
            returning_id.0 as u64,
        ))
    }

    pub fn fetch_component(
        &self,
        query: &UpdateFetchQuery,
    ) -> Result<FetchComponentModel<MarkdownComponent>, Report<MarkdownComponentServiceError>> {
        let component_data = self
            .cms_repository
            .fetch_component(query.id as i64)
            .change_context(MarkdownComponentServiceError::DbError)?
            .ok_or_else(|| {
                Report::new(MarkdownComponentServiceError::NotFound)
                    .attach(poem::http::StatusCode::NOT_FOUND)
            })?;

        Ok(FetchComponentModel {
            label: component_data.label,
            position: component_data.position,
            raw_data: component_data.raw_data.into(),
        })
    }

    pub fn update_component(
        &self,
        query: &UpdateFetchQuery,
        form: &MarkdownFormValidated,
    ) -> Result<(), Report<MarkdownComponentServiceError>> {
        self.cms_repository
            .update_component(UpdateComponentModel {
                id: query.id as i64,
                label: form.label.as_str().to_string(),
                raw_data: MarkdownComponent {
                    content: form.markdown.as_str().to_string(),
                    parse_html: form.markdown.as_str().to_string(),
                }
                .into_data(),
            })
            .change_context(MarkdownComponentServiceError::DbError)?;

        Ok(())
    }
}

impl FromContext for MarkdownComponentService {
    async fn from_context(ctx: &'_ Context<'_>) -> Result<Self, Report<ContextError>> {
        Ok(Self::new(ctx.inject().await?, ctx.inject().await?))
    }
}
