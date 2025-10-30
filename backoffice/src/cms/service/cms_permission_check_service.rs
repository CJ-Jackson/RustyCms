use crate::cms::repository::cms_repository::CmsRepository;
use crate::user::pointer::user_pointer::UserPointer;
use crate::user::role::Role;
use error_stack::{Report, ResultExt};
use poem::http::StatusCode;
use shared::utils::context::{Context, ContextError, FromContext};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CmsPermissionCheckServiceError {
    #[error("No permission")]
    NoPermission,
    #[error("Db error")]
    DbError,
    #[error("Not found error")]
    NotFound,
}

pub struct CmsPermissionCheckService {
    cms_repository: CmsRepository,
    user_pointer: UserPointer,
}

impl CmsPermissionCheckService {
    pub fn new(cms_repository: CmsRepository, user_pointer: UserPointer) -> Self {
        Self {
            cms_repository,
            user_pointer,
        }
    }

    pub fn check_permission_by_page_id(
        &self,
        page_id: i64,
    ) -> Result<(), Report<CmsPermissionCheckServiceError>> {
        let author_id = self
            .cms_repository
            .get_author_id_page(page_id)
            .change_context(CmsPermissionCheckServiceError::DbError)?
            .ok_or_else(|| {
                Report::new(CmsPermissionCheckServiceError::NotFound).attach(StatusCode::NOT_FOUND)
            })?;

        if self.user_pointer.role == Role::Root {
            return Ok(());
        }

        if author_id.0 != self.user_pointer.id {
            return Err(Report::new(CmsPermissionCheckServiceError::NoPermission)
                .attach(StatusCode::FORBIDDEN));
        }

        Ok(())
    }

    pub fn check_permission_by_component_id(
        &self,
        component_id: i64,
    ) -> Result<(), Report<CmsPermissionCheckServiceError>> {
        let author_id = self
            .cms_repository
            .get_author_id_component(component_id)
            .change_context(CmsPermissionCheckServiceError::DbError)?
            .ok_or_else(|| {
                Report::new(CmsPermissionCheckServiceError::NotFound).attach(StatusCode::NOT_FOUND)
            })?;

        if self.user_pointer.role == Role::Root {
            return Ok(());
        }

        if author_id.0 != self.user_pointer.id {
            return Err(Report::new(CmsPermissionCheckServiceError::NoPermission)
                .attach(StatusCode::FORBIDDEN));
        }

        Ok(())
    }
}

impl FromContext for CmsPermissionCheckService {
    async fn from_context(ctx: &'_ Context<'_>) -> Result<Self, Report<ContextError>> {
        Ok(Self::new(ctx.inject().await?, ctx.inject().await?))
    }
}
