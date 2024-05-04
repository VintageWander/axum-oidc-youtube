use axum::{
    async_trait,
    extract::{FromRequest, Request},
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use openidconnect::{core::CoreUserInfoClaims, reqwest::async_http_client, AccessToken};

use crate::{error::Error, AppState};

pub struct LoggedInUser(pub CoreUserInfoClaims);

#[async_trait]
impl FromRequest<AppState> for LoggedInUser {
    type Rejection = Error;
    async fn from_request(req: Request, state: &AppState) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req, state).await?;

        let access_token = AccessToken::new(bearer.token().into());

        let user_info: CoreUserInfoClaims = state
            .oidc
            .user_info(access_token, None)?
            .request_async(async_http_client)
            .await?;

        Ok(LoggedInUser(user_info))
    }
}
