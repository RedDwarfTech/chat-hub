use utoipa::{
    openapi::{
        self,
        security::{Http, HttpAuthScheme, SecurityScheme},
    },
    Modify, OpenApi,
};

use crate::types;
use crate::controller::chat::azure_controller;

#[derive(OpenApi)]
#[openapi(
    paths(
        azure_controller::ask
    ),
    components(
        schemas(
            utoipa::TupleUnit,
            types::GenericPostRequest,
            types::GenericPostResponse,
            types::GenericStringResponse,
            types::PostRequest,
            types::PostResponse,
        )
    ),
    tags((name = "BasicAPI", description = "A very Basic API")),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        // NOTE: we can unwrap safely since there already is components registered.
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "Token",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        );
    }
}
