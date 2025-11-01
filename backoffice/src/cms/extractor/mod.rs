use poem::{FromRequest, Request, RequestBody};

pub struct HeaderId(pub u64);

impl<'a> FromRequest<'a> for HeaderId {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> poem::Result<Self> {
        let value = req.header("X-Delete-Id").unwrap_or_default();
        let id = value.parse::<u64>().unwrap_or_default();
        Ok(HeaderId(id))
    }
}
