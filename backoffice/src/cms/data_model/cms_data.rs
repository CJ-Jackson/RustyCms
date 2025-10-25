use crate::cms::query_model::UpdateFetchQuery;
use chrono::{DateTime, Utc};
use shared::cms::markers::ComponentDataMarker;
use shared::cms::status::CmsPageStatus;

#[derive(Debug)]
pub struct ReturningIdModel(pub i64);

#[derive(Debug)]
pub struct UserIdModel(pub i64);

#[derive(Debug)]
pub struct AddPageModel {
    pub user_id: i64,
    pub title: String,
    pub status: CmsPageStatus,
}

#[derive(Debug)]
pub struct CreateComponentModel<T: ComponentDataMarker = Vec<u8>> {
    pub page_id: i64,
    pub kind_uuid: String,
    pub raw_data: T,
    pub label: String,
}

#[derive(Debug)]
pub struct FetchComponentModel<T: ComponentDataMarker = Vec<u8>> {
    pub label: String,
    pub position: i64,
    pub raw_data: T,
}

pub struct FetchPageModel {
    #[allow(dead_code)]
    pub id: i64,
    pub title: String,
    pub summary: String,
    pub status: CmsPageStatus,
}

#[derive(Debug)]
pub struct ListComponentModel {
    pub id: i64,
    pub kind_uuid: String,
    pub position: i64,
    pub label: String,
}

impl ListComponentModel {
    pub fn as_query(&self) -> UpdateFetchQuery {
        UpdateFetchQuery::new(self.kind_uuid.clone(), self.id as u64)
    }
}

#[derive(Debug)]
pub struct ListPageModel {
    pub id: i64,
    pub author: String,
    pub user_id: i64,
    pub title: String,
    pub added: DateTime<Utc>,
    pub updated: Option<DateTime<Utc>>,
    pub status: CmsPageStatus,
}

#[derive(Debug)]
pub struct UpdateComponentModel<T: ComponentDataMarker = Vec<u8>> {
    pub id: i64,
    pub label: String,
    pub raw_data: T,
}

#[derive(Debug)]
pub struct UpdateComponentPositionModel {
    pub id: i64,
    pub position: i64,
}

#[derive(Debug)]
pub struct UpdatePageModel {
    pub id: i64,
    pub title: String,
    pub summary: String,
    pub status: CmsPageStatus,
}
