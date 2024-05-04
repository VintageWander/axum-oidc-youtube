use crate::user::*;
use utoipa::{
    openapi::{
        self,
        security::{OpenIdConnect, SecurityScheme},
    },
    Modify, OpenApi, ToSchema,
};

use crate::env::issuer_url;

#[derive(OpenApi)]
#[openapi(
    paths(
        profile
    ),
    components(
        schemas(Test)
    ), 
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "Open ID Connect",
                SecurityScheme::OpenIdConnect(OpenIdConnect::new(format!(
                    "{}.well-known/openid-configuration",
                    *issuer_url()
                ))),
            )
        }
    }
}

#[derive(ToSchema)]
struct Test;
