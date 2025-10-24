use crate::cms::data_model::cms_data::{
    AddPageModel, FetchPageModel, ListComponentModel, ListPageModel, ReturningIdModel,
    UpdateComponentPositionModel, UpdatePageModel,
};
use crate::cms::form::add_page_form::AddPageFormValidated;
use crate::cms::form::amend_page_form::AmendPageFormValidated;
use crate::cms::form::component_position_form::ComponentPositionForm;
use crate::cms::repository::cms_repository::CmsRepository;
use crate::user::pointer::user_pointer::UserPointer;
use error_stack::{Report, ResultExt};
use poem::http::StatusCode;
use shared::context::{Context, ContextError, FromContext};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CmsPageServiceError {
    #[error("Db error")]
    DbError,
    #[error("Not found error")]
    NotFoundError,
}

pub struct CmsPageService {
    cms_repository: CmsRepository,
    user_pointer: UserPointer,
}

impl CmsPageService {
    pub fn new(cms_repository: CmsRepository, user_pointer: UserPointer) -> Self {
        Self {
            cms_repository,
            user_pointer,
        }
    }

    pub fn add_page(
        &self,
        add_page_form: &AddPageFormValidated,
    ) -> Result<ReturningIdModel, Report<CmsPageServiceError>> {
        self.cms_repository
            .add_page(AddPageModel {
                user_id: self.user_pointer.id,
                title: add_page_form.title.as_str().to_string(),
                summary: "".to_string(),
                status: Default::default(),
            })
            .change_context(CmsPageServiceError::DbError)
    }

    pub fn fetch_page(&self, page_id: i64) -> Result<FetchPageModel, Report<CmsPageServiceError>> {
        self.cms_repository
            .fetch_page(page_id)
            .change_context(CmsPageServiceError::DbError)?
            .ok_or_else(|| {
                Report::new(CmsPageServiceError::NotFoundError).attach(StatusCode::NOT_FOUND)
            })
    }

    pub fn list_page(&self) -> Result<Arc<[ListPageModel]>, Report<CmsPageServiceError>> {
        self.cms_repository
            .list_page()
            .change_context(CmsPageServiceError::DbError)
    }

    pub fn update_page(
        &self,
        page_id: i64,
        amend_page_form: &AmendPageFormValidated,
    ) -> Result<(), Report<CmsPageServiceError>> {
        self.cms_repository
            .update_page(UpdatePageModel {
                id: page_id,
                title: amend_page_form.title.as_str().to_string(),
                summary: amend_page_form.summary.as_str().to_string(),
                status: amend_page_form.status.clone(),
            })
            .change_context(CmsPageServiceError::DbError)
    }

    pub fn update_component_position(
        &self,
        component_position_form: &ComponentPositionForm,
    ) -> Result<(), Report<CmsPageServiceError>> {
        for (component_id, position) in component_position_form.position.iter() {
            self.cms_repository
                .update_component_position(UpdateComponentPositionModel {
                    id: *component_id as i64,
                    position: *position as i64,
                })
                .change_context(CmsPageServiceError::DbError)?;
        }
        Ok(())
    }

    pub fn delete_component(&self, component_id: i64) -> Result<(), Report<CmsPageServiceError>> {
        self.cms_repository
            .delete_component(component_id)
            .change_context(CmsPageServiceError::DbError)
    }

    pub fn list_component(
        &self,
        page_id: i64,
    ) -> Result<Arc<[ListComponentModel]>, Report<CmsPageServiceError>> {
        self.cms_repository
            .list_component(page_id)
            .change_context(CmsPageServiceError::DbError)
    }
}

impl FromContext for CmsPageService {
    async fn from_context(ctx: &'_ Context<'_>) -> Result<Self, Report<ContextError>> {
        Ok(Self::new(ctx.inject().await?, ctx.inject().await?))
    }
}
