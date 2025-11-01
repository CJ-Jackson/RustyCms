use crate::cms::data_model::cms_data::{
    AddFileAttachmentModel, AddPageModel, CreateComponentModel, FetchComponentModel,
    FetchPageModel, FilePath, ListComponentModel, ListFileAttachmentModel, ListPageModel,
    ReturningIdModel, UpdateComponentModel, UpdateComponentPositionModel, UpdatePageModel,
    UserIdModel,
};
use error_stack::{Report, ResultExt};
use poem::http::StatusCode;
use rusqlite::{Connection, OptionalExtension, named_params};
use shared::cms::status::CmsPageStatus;
use shared::utils::context::{Context, ContextError, FromContext};
use shared::utils::db::{BorrowConnectionExt, SqliteClient};
use shared::utils::error::ExtraResultExt;
use std::sync::{Arc, MutexGuard};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CmsRepositoryError {
    #[error("Query error")]
    QueryError,
    #[error("Row Value error")]
    RowValueError,
    #[error("Borrow Conn error")]
    BorrowConnError,
}

pub struct CmsRepository {
    sqlite_client: Option<SqliteClient>,
}

impl CmsRepository {
    pub fn new(sqlite_client: SqliteClient) -> Self {
        Self {
            sqlite_client: Some(sqlite_client),
        }
    }

    fn borrow_conn(&self) -> Result<MutexGuard<'_, Connection>, Report<CmsRepositoryError>> {
        self.sqlite_client
            .borrow_conn()
            .change_context(CmsRepositoryError::BorrowConnError)
    }
}

impl CmsRepository {
    pub fn add_file_attachment(
        &self,
        add_file_attachment_model: AddFileAttachmentModel,
    ) -> Result<(), Report<CmsRepositoryError>> {
        let conn = self.borrow_conn()?;

        conn.execute(
            include_str!("_sql/cms_repository/add_file_attachment.sql"),
            named_params! {
                ":component_id": add_file_attachment_model.component_id,
                ":file_name": add_file_attachment_model.file_name,
                ":file_path": add_file_attachment_model.file_path,
                ":file_type": add_file_attachment_model.file_type,
            },
        )
        .change_context(CmsRepositoryError::QueryError)
        .attach(StatusCode::INTERNAL_SERVER_ERROR)
        .log_it()?;

        Ok(())
    }

    pub fn add_page(
        &self,
        page: AddPageModel,
    ) -> Result<ReturningIdModel, Report<CmsRepositoryError>> {
        let conn = self.borrow_conn()?;

        let mut stmt = conn
            .prepare(include_str!("_sql/cms_repository/add_page.sql"))
            .change_context(CmsRepositoryError::QueryError)
            .attach(StatusCode::INTERNAL_SERVER_ERROR)
            .log_it()?;

        let row: ReturningIdModel = stmt
            .query_one(
                named_params! {
                    ":user_id": page.user_id,
                    ":title": page.title,
                    ":status": page.status.as_stringed(),
                },
                |row| Ok(ReturningIdModel(row.get("id")?)),
            )
            .change_context(CmsRepositoryError::RowValueError)
            .attach(StatusCode::INTERNAL_SERVER_ERROR)
            .log_it()?;

        Ok(row)
    }

    pub fn create_component(
        &self,
        create_component_model: CreateComponentModel,
    ) -> Result<ReturningIdModel, Report<CmsRepositoryError>> {
        let conn = self.borrow_conn()?;

        let mut stmt = conn
            .prepare(include_str!("_sql/cms_repository/create_component.sql"))
            .change_context(CmsRepositoryError::QueryError)
            .attach(StatusCode::INTERNAL_SERVER_ERROR)
            .log_it()?;

        let row: ReturningIdModel = stmt
            .query_one(
                named_params! {
                    ":page_id": create_component_model.page_id,
                    ":kind_uuid": create_component_model.kind_uuid,
                    ":raw_data": create_component_model.raw_data,
                    ":label": create_component_model.label,
                },
                |row| Ok(ReturningIdModel(row.get("id")?)),
            )
            .change_context(CmsRepositoryError::RowValueError)
            .attach(StatusCode::INTERNAL_SERVER_ERROR)
            .log_it()?;

        Ok(row)
    }

    pub fn delete_component(&self, id: i64) -> Result<(), Report<CmsRepositoryError>> {
        let conn = self.borrow_conn()?;

        conn.execute(
            include_str!("_sql/cms_repository/delete_component.sql"),
            named_params! {
                ":id": id,
            },
        )
        .change_context(CmsRepositoryError::QueryError)
        .attach(StatusCode::INTERNAL_SERVER_ERROR)
        .log_it()?;

        Ok(())
    }

    pub fn delete_file_attachment(&self, id: i64) -> Result<(), Report<CmsRepositoryError>> {
        let conn = self.borrow_conn()?;

        conn.execute(
            include_str!("_sql/cms_repository/delete_file_attachment.sql"),
            named_params! {
                ":id": id,
            },
        )
        .change_context(CmsRepositoryError::QueryError)
        .attach(StatusCode::INTERNAL_SERVER_ERROR)
        .log_it()?;

        Ok(())
    }

    pub fn delete_file_attachment_by_component_id(
        &self,
        component_id: i64,
    ) -> Result<(), Report<CmsRepositoryError>> {
        let conn = self.borrow_conn()?;

        conn.execute(
            include_str!("_sql/cms_repository/delele_file_attachment_component.sql"),
            named_params! {
                ":component_id": component_id,
            },
        )
        .change_context(CmsRepositoryError::QueryError)
        .attach(StatusCode::INTERNAL_SERVER_ERROR)
        .log_it()?;

        Ok(())
    }

    pub fn fetch_component(
        &self,
        id: i64,
    ) -> Result<Option<FetchComponentModel>, Report<CmsRepositoryError>> {
        let conn = self.borrow_conn()?;

        let mut stmt = conn
            .prepare(include_str!("_sql/cms_repository/fetch_component.sql"))
            .change_context(CmsRepositoryError::QueryError)
            .attach(StatusCode::INTERNAL_SERVER_ERROR)
            .log_it()?;

        let row: Option<FetchComponentModel> = stmt
            .query_one(
                named_params! {
                    ":id": id,
                },
                |row| {
                    Ok(FetchComponentModel {
                        label: row.get("label")?,
                        position: row.get("position")?,
                        raw_data: row.get("raw_data")?,
                    })
                },
            )
            .optional()
            .change_context(CmsRepositoryError::RowValueError)
            .attach(StatusCode::INTERNAL_SERVER_ERROR)
            .log_it()?;

        Ok(row)
    }

    pub fn fetch_page(
        &self,
        id: i64,
    ) -> Result<Option<FetchPageModel>, Report<CmsRepositoryError>> {
        let conn = self.borrow_conn()?;

        let mut stmt = conn
            .prepare(include_str!("_sql/cms_repository/fetch_page.sql"))
            .change_context(CmsRepositoryError::QueryError)
            .attach(StatusCode::INTERNAL_SERVER_ERROR)
            .log_it()?;

        let row: Option<FetchPageModel> = stmt
            .query_one(
                named_params! {
                    ":id": id,
                },
                |row| {
                    Ok(FetchPageModel {
                        id: row.get("id")?,
                        title: row.get("title")?,
                        summary: row.get("summary")?,
                        status: CmsPageStatus::try_from(row.get::<_, String>("status")?)
                            .unwrap_or_default(),
                    })
                },
            )
            .optional()
            .change_context(CmsRepositoryError::RowValueError)
            .attach(StatusCode::INTERNAL_SERVER_ERROR)
            .log_it()?;

        Ok(row)
    }

    pub fn get_author_id_component(
        &self,
        id: i64,
    ) -> Result<Option<UserIdModel>, Report<CmsRepositoryError>> {
        let conn = self.borrow_conn()?;

        let mut stmt = conn
            .prepare(include_str!(
                "_sql/cms_repository/get_author_id_component.sql"
            ))
            .change_context(CmsRepositoryError::QueryError)
            .attach(StatusCode::INTERNAL_SERVER_ERROR)
            .log_it()?;

        let row: Option<UserIdModel> = stmt
            .query_one(
                named_params! {
                    ":id": id,
                },
                |row| Ok(UserIdModel(row.get("user_id")?)),
            )
            .optional()
            .change_context(CmsRepositoryError::RowValueError)
            .attach(StatusCode::INTERNAL_SERVER_ERROR)
            .log_it()?;

        Ok(row)
    }

    pub fn get_author_id_page(
        &self,
        id: i64,
    ) -> Result<Option<UserIdModel>, Report<CmsRepositoryError>> {
        let conn = self.borrow_conn()?;

        let mut stmt = conn
            .prepare(include_str!("_sql/cms_repository/get_author_id_page.sql"))
            .change_context(CmsRepositoryError::QueryError)
            .attach(StatusCode::INTERNAL_SERVER_ERROR)
            .log_it()?;

        let row: Option<UserIdModel> = stmt
            .query_one(
                named_params! {
                    ":id": id,
                },
                |row| Ok(UserIdModel(row.get("user_id")?)),
            )
            .optional()
            .change_context(CmsRepositoryError::RowValueError)
            .attach(StatusCode::INTERNAL_SERVER_ERROR)
            .log_it()?;

        Ok(row)
    }

    pub fn get_file_path(
        &self,
        id: i64,
        component_id: i64,
    ) -> Result<Option<FilePath>, Report<CmsRepositoryError>> {
        let conn = self.borrow_conn()?;

        let mut stmt = conn
            .prepare(include_str!("_sql/cms_repository/get_file_path.sql"))
            .change_context(CmsRepositoryError::QueryError)
            .attach(StatusCode::INTERNAL_SERVER_ERROR)
            .log_it()?;

        let row: Option<FilePath> = stmt
            .query_one(
                named_params! {
                    ":id": id,
                    ":component_id": component_id,
                },
                |row| Ok(FilePath(row.get("file_path")?)),
            )
            .optional()
            .change_context(CmsRepositoryError::RowValueError)
            .attach(StatusCode::INTERNAL_SERVER_ERROR)
            .log_it()?;

        Ok(row)
    }

    pub fn list_component(
        &self,
        page_id: i64,
    ) -> Result<Arc<[ListComponentModel]>, Report<CmsRepositoryError>> {
        let conn = self.borrow_conn()?;

        let mut stmt = conn
            .prepare(include_str!("_sql/cms_repository/list_component.sql"))
            .change_context(CmsRepositoryError::QueryError)
            .attach(StatusCode::INTERNAL_SERVER_ERROR)
            .log_it()?;

        let rows = stmt
            .query_map(
                named_params! {
                    ":page_id": page_id,
                },
                |row| {
                    Ok(ListComponentModel {
                        id: row.get("id")?,
                        kind_uuid: row.get("kind_uuid")?,
                        position: row.get("position")?,
                        label: row.get("label")?,
                    })
                },
            )
            .change_context(CmsRepositoryError::RowValueError)
            .attach(StatusCode::INTERNAL_SERVER_ERROR)
            .log_it()?;

        let rows = rows
            .collect::<Result<Vec<_>, _>>()
            .change_context(CmsRepositoryError::RowValueError)
            .attach(StatusCode::INTERNAL_SERVER_ERROR)
            .log_it()?;

        Ok(rows.into())
    }

    pub fn list_file_attachment(
        &self,
        component_id: i64,
    ) -> Result<Arc<[ListFileAttachmentModel]>, Report<CmsRepositoryError>> {
        let conn = self.borrow_conn()?;

        let mut stmt = conn
            .prepare(include_str!(
                "_sql/cms_repository/list_file_attachment_component.sql"
            ))
            .change_context(CmsRepositoryError::QueryError)
            .attach(StatusCode::INTERNAL_SERVER_ERROR)
            .log_it()?;

        let rows = stmt
            .query_map(
                named_params! {
                    ":component_id": component_id,
                },
                |row| {
                    Ok(ListFileAttachmentModel {
                        id: row.get("id")?,
                        file_name: row.get("file_name")?,
                        file_path: row.get("file_path")?,
                        file_type: row.get("file_type")?,
                        uploaded: row.get("uploaded")?,
                    })
                },
            )
            .change_context(CmsRepositoryError::RowValueError)
            .attach(StatusCode::INTERNAL_SERVER_ERROR)
            .log_it()?;

        let rows = rows
            .collect::<Result<Vec<_>, _>>()
            .change_context(CmsRepositoryError::RowValueError)
            .attach(StatusCode::INTERNAL_SERVER_ERROR)
            .log_it()?;

        Ok(rows.into())
    }

    pub fn list_page(&self) -> Result<Arc<[ListPageModel]>, Report<CmsRepositoryError>> {
        let conn = self.borrow_conn()?;

        let mut stmt = conn
            .prepare(include_str!("_sql/cms_repository/list_page.sql"))
            .change_context(CmsRepositoryError::QueryError)
            .log_it()?;

        let rows = stmt
            .query_map(named_params! {}, |row| {
                Ok(ListPageModel {
                    id: row.get("id")?,
                    author: row.get("author")?,
                    user_id: row.get("user_id")?,
                    title: row.get("title")?,
                    added: row.get("added")?,
                    updated: row.get("updated")?,
                    status: CmsPageStatus::try_from(row.get::<_, String>("status")?)
                        .unwrap_or_default(),
                })
            })
            .change_context(CmsRepositoryError::RowValueError)
            .attach(StatusCode::INTERNAL_SERVER_ERROR)
            .log_it()?;

        let rows = rows
            .collect::<Result<Vec<_>, _>>()
            .change_context(CmsRepositoryError::RowValueError)
            .attach(StatusCode::INTERNAL_SERVER_ERROR)
            .log_it()?;

        Ok(rows.into())
    }

    pub fn update_component(
        &self,
        update_component_model: UpdateComponentModel,
    ) -> Result<(), Report<CmsRepositoryError>> {
        let conn = self.borrow_conn()?;

        conn.execute(
            include_str!("_sql/cms_repository/update_component.sql"),
            named_params! {
                ":label": update_component_model.label,
                ":raw_data": update_component_model.raw_data,
                ":id": update_component_model.id,
            },
        )
        .change_context(CmsRepositoryError::QueryError)
        .attach(StatusCode::INTERNAL_SERVER_ERROR)
        .log_it()?;

        Ok(())
    }

    pub fn update_component_position(
        &self,
        update_component_position_model: UpdateComponentPositionModel,
    ) -> Result<(), Report<CmsRepositoryError>> {
        let conn = self.borrow_conn()?;

        conn.execute(
            include_str!("_sql/cms_repository/update_component_position.sql"),
            named_params! {
                ":id": update_component_position_model.id,
                ":position": update_component_position_model.position,
                ":page_id": update_component_position_model.page_id,
            },
        )
        .change_context(CmsRepositoryError::QueryError)
        .attach(StatusCode::INTERNAL_SERVER_ERROR)
        .log_it()?;

        Ok(())
    }

    pub fn update_page(
        &self,
        update_page_model: UpdatePageModel,
    ) -> Result<(), Report<CmsRepositoryError>> {
        let conn = self.borrow_conn()?;

        conn.execute(
            include_str!("_sql/cms_repository/update_page.sql"),
            named_params! {
                ":title": update_page_model.title,
                ":summary": update_page_model.summary,
                ":status": update_page_model.status.as_stringed(),
                ":id": update_page_model.id,
            },
        )
        .change_context(CmsRepositoryError::QueryError)
        .attach(StatusCode::INTERNAL_SERVER_ERROR)
        .log_it()?;

        Ok(())
    }
}

impl FromContext for CmsRepository {
    async fn from_context(ctx: &'_ Context<'_>) -> Result<Self, Report<ContextError>> {
        Ok(Self::new(ctx.inject().await?))
    }
}
