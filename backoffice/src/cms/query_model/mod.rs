use poem::{FromRequest, Request, RequestBody};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CreateQuery {
    pub kind_uuid: String,
    pub page_id: u64,
}

impl CreateQuery {
    pub fn new(kind_uuid: String, page_id: u64) -> Self {
        Self { kind_uuid, page_id }
    }

    pub fn query_string(&self) -> String {
        serde_qs::to_string(self).expect("failed to serialize query string")
    }
}

impl<'a> FromRequest<'a> for CreateQuery {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> poem::Result<Self> {
        let query = req
            .data::<Self>()
            .ok_or_else(|| poem::Error::from_status(poem::http::StatusCode::BAD_REQUEST))?;
        Ok(query.clone())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UpdateFetchQuery {
    pub kind_uuid: String,
    pub id: u64,
}

impl UpdateFetchQuery {
    pub fn new(kind_uuid: String, id: u64) -> Self {
        Self { kind_uuid, id }
    }

    pub fn query_string(&self) -> String {
        serde_qs::to_string(self).expect("failed to serialize query string")
    }
}

impl<'a> FromRequest<'a> for UpdateFetchQuery {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> poem::Result<Self> {
        let query = req
            .data::<Self>()
            .ok_or_else(|| poem::Error::from_status(poem::http::StatusCode::BAD_REQUEST))?;
        Ok(query.clone())
    }
}
