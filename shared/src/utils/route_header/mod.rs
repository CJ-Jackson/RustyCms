use poem::endpoint::BoxEndpoint;
use poem::http::StatusCode;
use poem::{Endpoint, EndpointExt, IntoEndpoint, Request, Response};
use std::collections::HashMap;

pub struct RouteHeader {
    header_name: String,
    default_route: BoxEndpoint<'static>,
    routes: HashMap<&'static str, BoxEndpoint<'static>>,
}

impl RouteHeader {
    pub fn new<E>(header_name: String, default_route: E) -> Self
    where
        E: IntoEndpoint,
        E::Endpoint: 'static,
    {
        Self {
            header_name,
            default_route: default_route.into_endpoint().map_to_response().boxed(),
            routes: HashMap::new(),
        }
    }

    pub fn at<E>(mut self, route: &'static str, endpoint: E) -> Self
    where
        E: IntoEndpoint,
        E::Endpoint: 'static,
    {
        self.routes
            .insert(route, endpoint.into_endpoint().map_to_response().boxed());
        self
    }
}

pub fn route_header<E>(header_name: String, default_route: E) -> RouteHeader
where
    E: IntoEndpoint,
    E::Endpoint: 'static,
{
    RouteHeader::new(header_name, default_route)
}

impl Endpoint for RouteHeader {
    type Output = Response;

    async fn call(&self, req: Request) -> poem::Result<Self::Output> {
        match req.header(self.header_name.as_str()) {
            None => self.default_route.call(req).await,
            Some(header_value) => {
                if let Some(route) = self.routes.get(header_value.trim()) {
                    route.call(req).await
                } else {
                    Err(poem::Error::from_status(StatusCode::NOT_FOUND))
                }
            }
        }
    }
}
