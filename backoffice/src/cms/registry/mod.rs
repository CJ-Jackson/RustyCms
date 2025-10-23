use crate::cms::enums::ComponentRequestKind;
use crate::cms::methods::ComponentMethods;
use crate::cms::query_model::{CreateQuery, UpdateFetchQuery};
use crate::cms::route::component::markdown::markdown_registry_item;
use crate::cms::service::cms_permission_check_service::{
    CmsPermissionCheckService, CmsPermissionCheckServiceError,
};
use poem::http::StatusCode;
use poem::{Endpoint, FromRequest, IntoEndpoint, Request};
use shared::cms::CmsComponentInfo;
use shared::context::Dep;
use shared::error::FromErrorStack;
use shared::query_string::query::QueryQs;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

struct Registry(Arc<HashMap<String, ComponentMethods>>);

impl Clone for Registry {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

static REGISTRY: OnceLock<Registry> = OnceLock::new();

fn insert(map: &mut HashMap<String, ComponentMethods>, item: ComponentMethods) {
    map.insert(item.info.kind_uuid.clone(), item);
}

fn registry() -> Registry {
    let registry = REGISTRY.get_or_init(|| {
        let mut map: HashMap<String, ComponentMethods> = HashMap::new();

        insert(&mut map, markdown_registry_item());

        Registry(Arc::new(map))
    });

    registry.clone()
}

static REGISTRY_ITEM: OnceLock<Arc<[CmsComponentInfo]>> = OnceLock::new();

pub fn registry_item() -> Arc<[CmsComponentInfo]> {
    REGISTRY_ITEM
        .get_or_init(|| {
            let registry = registry();
            let mut items: Vec<CmsComponentInfo> =
                registry.0.values().map(|item| item.info.clone()).collect();
            items.sort_by(|a, b| a.kind.cmp(&b.kind));

            items.into_iter().collect()
        })
        .clone()
}

struct RegistryEndpoint {
    registry: Registry,
    request_kind: ComponentRequestKind,
}

impl Endpoint for RegistryEndpoint {
    type Output = poem::Response;

    async fn call(&self, mut req: Request) -> poem::Result<Self::Output> {
        let Dep(cmd_permission_check_service) =
            Dep::<CmsPermissionCheckService>::from_request_without_body(&req).await?;
        match self.request_kind {
            ComponentRequestKind::Create => {
                let query = QueryQs::<CreateQuery>::from_request_without_body(&req)
                    .await?
                    .0;
                cmd_permission_check_service
                    .check_permission_by_page_id(query.page_id as i64)
                    .map_err(poem::Error::from_error_stack)?;
                let item = self
                    .registry
                    .0
                    .get(&query.kind_uuid)
                    .ok_or_else(|| poem::Error::from_status(StatusCode::NOT_FOUND))?;
                req.set_data(item.info.clone());
                req.set_data(query);
                item.create.call(req).await
            }
            ComponentRequestKind::UpdateFetch => {
                let query = QueryQs::<UpdateFetchQuery>::from_request_without_body(&req)
                    .await?
                    .0;
                cmd_permission_check_service
                    .check_permission_by_component_id(query.id as i64)
                    .map_err(poem::Error::from_error_stack)?;
                let item = self
                    .registry
                    .0
                    .get(&query.kind_uuid)
                    .ok_or_else(|| poem::Error::from_status(StatusCode::NOT_FOUND))?;
                req.set_data(item.info.clone());
                req.set_data(query);
                item.update_fetch.call(req).await
            }
        }
    }
}

pub fn registry_ep_create() -> impl IntoEndpoint {
    RegistryEndpoint {
        registry: registry(),
        request_kind: ComponentRequestKind::Create,
    }
}

pub fn registry_ep_update_fetch() -> impl IntoEndpoint {
    RegistryEndpoint {
        registry: registry(),
        request_kind: ComponentRequestKind::UpdateFetch,
    }
}
