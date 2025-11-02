use crate::cms::data_model::cms_data::{AddFileAttachmentModel, ListFileAttachmentModel};
use crate::cms::repository::cms_repository::CmsRepository;
use crate::user::pointer::user_pointer::UserPointer;
use chrono::{DateTime, Utc};
use error_stack::{Report, ResultExt};
use poem::web::Field;
use shared::utils::config::ConfigPointer;
use shared::utils::context::{Context, ContextError, FromContext};
use std::path::Path;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CmsAttachmentServiceError {
    #[error("Db error")]
    DbError,
    #[error("Save error")]
    SaveError,
    #[error("Not found error")]
    NotFoundError,
}

pub struct CmsAttachmentService {
    file_upload_path: String,
    cms_repository: CmsRepository,
    user_pointer: UserPointer,
    stamp: DateTime<Utc>,
}

impl CmsAttachmentService {
    const SAVE_PATH: &'static str = "/cms/files";

    pub fn new(
        file_upload_path: String,
        cms_repository: CmsRepository,
        user_pointer: UserPointer,
    ) -> Self {
        Self {
            file_upload_path,
            cms_repository,
            user_pointer,
            stamp: Utc::now(),
        }
    }

    pub async fn add_file(
        &self,
        component_id: i64,
        field: Field,
    ) -> Result<(), Report<CmsAttachmentServiceError>> {
        let file_name = field.file_name().unwrap_or_default().to_string();
        let file_type = field.content_type().unwrap_or_default().to_string();
        let file_path = format!(
            "{}/{}-{}/{}",
            Self::SAVE_PATH,
            self.stamp.format("%Y-%m-%d-%s"),
            self.user_pointer.id,
            &file_name
        );
        let save_file_path = format!("{}/{}", &self.file_upload_path, &file_path);

        let file_content = field
            .bytes()
            .await
            .change_context(CmsAttachmentServiceError::SaveError)
            .attach(poem::http::StatusCode::BAD_REQUEST)?;

        // save file path dirname
        let save_file_path = Path::new(&save_file_path);

        _ = std::fs::create_dir_all(save_file_path.parent().ok_or_else(|| {
            Report::new(CmsAttachmentServiceError::SaveError)
                .attach(poem::http::StatusCode::BAD_REQUEST)
        })?);

        std::fs::write(save_file_path, file_content)
            .change_context(CmsAttachmentServiceError::SaveError)
            .attach(poem::http::StatusCode::BAD_REQUEST)?;

        self.cms_repository
            .add_file_attachment(AddFileAttachmentModel {
                component_id,
                file_name,
                file_path,
                file_type,
            })
            .change_context(CmsAttachmentServiceError::DbError)
            .attach(poem::http::StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(())
    }

    pub fn delete_file_by_component_id(
        &self,
        component_id: i64,
    ) -> Result<(), Report<CmsAttachmentServiceError>> {
        let list_file = self
            .cms_repository
            .list_file_attachment(component_id)
            .change_context(CmsAttachmentServiceError::DbError)
            .attach(poem::http::StatusCode::INTERNAL_SERVER_ERROR)?;

        self.cms_repository
            .delete_file_attachment_by_component_id(component_id)
            .change_context(CmsAttachmentServiceError::DbError)
            .attach(poem::http::StatusCode::INTERNAL_SERVER_ERROR)?;

        for file in list_file.iter() {
            let file_path = format!("{}/{}", &self.file_upload_path, &file.file_path);
            _ = std::fs::remove_file(file_path);
        }

        Ok(())
    }

    pub fn delete_file_by_id(
        &self,
        id: i64,
        component_id: i64,
    ) -> Result<(), Report<CmsAttachmentServiceError>> {
        let file = self
            .cms_repository
            .get_file_path(id, component_id)
            .change_context(CmsAttachmentServiceError::DbError)
            .attach(poem::http::StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or_else(|| {
                Report::new(CmsAttachmentServiceError::NotFoundError)
                    .attach(poem::http::StatusCode::NOT_FOUND)
            })?;

        self.cms_repository
            .delete_file_attachment(id)
            .change_context(CmsAttachmentServiceError::DbError)
            .attach(poem::http::StatusCode::INTERNAL_SERVER_ERROR)?;

        let file_path = format!("{}/{}", &self.file_upload_path, &file.0);
        _ = std::fs::remove_file(file_path);

        Ok(())
    }

    pub fn list_file_by_component_id(
        &self,
        component_id: i64,
    ) -> Result<Arc<[ListFileAttachmentModel]>, Report<CmsAttachmentServiceError>> {
        self.cms_repository
            .list_file_attachment(component_id)
            .change_context(CmsAttachmentServiceError::DbError)
            .attach(poem::http::StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl FromContext for CmsAttachmentService {
    async fn from_context(ctx: &'_ Context<'_>) -> Result<Self, Report<ContextError>> {
        let config: ConfigPointer = ctx.inject().await?;
        Ok(Self::new(
            config.file_path.clone(),
            ctx.inject().await?,
            ctx.inject().await?,
        ))
    }
}
