use crate::cms::data_model::cms_data::AddFileAttachmentModel;
use crate::cms::repository::cms_repository::CmsRepository;
use error_stack::{Report, ResultExt};
use poem::web::Field;
use shared::utils::config::ConfigPointer;
use shared::utils::context::{Context, ContextError, FromContext};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CmsAttachmentServiceError {
    #[error("Db error")]
    DbError,
    #[error("Save error")]
    SaveError,
}

pub struct CmsAttachmentService {
    file_upload_path: String,
    cms_repository: CmsRepository,
}

impl CmsAttachmentService {
    pub const SAVE_PATH: &'static str = "cms/files";

    pub fn new(file_upload_path: String, cms_repository: CmsRepository) -> Self {
        Self {
            file_upload_path,
            cms_repository,
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
            "{}/{}/{}",
            Self::SAVE_PATH,
            chrono::Utc::now().format("%Y/%m"),
            &file_name
        );
        let save_file_path = format!("{}/{}", &self.file_upload_path, &file_path);

        let file_content = field
            .bytes()
            .await
            .change_context(CmsAttachmentServiceError::SaveError)
            .attach(poem::http::StatusCode::BAD_REQUEST)?;

        _ = std::fs::create_dir_all(format!("{}/{}", &self.file_upload_path, Self::SAVE_PATH));

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
}

impl FromContext for CmsAttachmentService {
    async fn from_context(ctx: &'_ Context<'_>) -> Result<Self, Report<ContextError>> {
        let config: ConfigPointer = ctx.inject().await?;
        Ok(Self::new(config.file_path.clone(), ctx.inject().await?))
    }
}
