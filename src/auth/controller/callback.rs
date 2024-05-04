use axum::{
    extract::{Query, State},
    routing::get,
    Router,
};
use openidconnect::{
    core::CoreUserInfoClaims, reqwest::async_http_client, AccessTokenHash, AuthorizationCode,
    Nonce, OAuth2TokenResponse, PkceCodeVerifier, TokenResponse,
};
use serde::Deserialize;

use crate::{auth::model::OidcStateTable, error::Error, web::Web, AppState, WebResult};

#[derive(Deserialize)]
struct CallbackQuery {
    pub code: String,  // Auth code
    pub state: String, // Csrf token
}

pub fn callback() -> Router<AppState> {
    async fn handler(
        State(AppState { db, oidc }): State<AppState>,
        Query(CallbackQuery { code, state }): Query<CallbackQuery>,
    ) -> WebResult {
        let OidcStateTable {
            csrf_token,
            code_verifier,
            nonce,
        } = sqlx::query_as!(
            OidcStateTable,
            "delete from oidc_state where csrf_token = $1 returning *;",
            state
        )
        .fetch_one(&db)
        .await?;

        let token_response = oidc
            .exchange_code(AuthorizationCode::new(code))
            .set_pkce_verifier(PkceCodeVerifier::new(code_verifier))
            .request_async(async_http_client)
            .await?;

        let id_token = token_response.id_token().ok_or(Error::MissingToken)?;

        let claims = id_token.claims(&oidc.id_token_verifier(), &Nonce::new(nonce))?;

        if let Some(expected_access_token_hash) = claims.access_token_hash() {
            let actual_access_token_hash = AccessTokenHash::from_token(
                token_response.access_token(),
                &id_token.signing_alg()?,
            )?;
            if actual_access_token_hash != *expected_access_token_hash {
                return Err(Error::AccessTokenMismatch);
            }
        }

        let user_info: CoreUserInfoClaims = oidc
            .user_info(token_response.access_token().to_owned(), None)?
            .request_async(async_http_client)
            .await?;

        Ok(Web::ok("Login successfully", user_info))
    }
    Router::new().route("/callback", get(handler))
}
