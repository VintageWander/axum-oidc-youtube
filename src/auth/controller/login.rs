use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use openidconnect::{
    core::{CoreAuthPrompt, CoreAuthenticationFlow},
    CsrfToken, Nonce, PkceCodeChallenge, Scope,
};

use crate::{web::Web, AppState, WebResult};

pub fn login() -> Router<AppState> {
    async fn handler(State(AppState { db, oidc }): State<AppState>) -> WebResult {
        let (code_challenge, code_verifier) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, csrf_token, nonce) = oidc
            .authorize_url(
                CoreAuthenticationFlow::AuthorizationCode,
                CsrfToken::new_random,
                Nonce::new_random,
            )
            .add_scopes([
                Scope::new("openid".into()),
                Scope::new("profile".into()),
                Scope::new("email".into()),
                Scope::new("offline_access".into()),
            ])
            .add_prompt(CoreAuthPrompt::Consent)
            .set_pkce_challenge(code_challenge)
            .url();

        sqlx::query!(
            "
            insert into oidc_state 
                (csrf_token, code_verifier, nonce)
            values
                ($1, $2, $3)
            ",
            csrf_token.secret().to_string(),
            code_verifier.secret().to_string(),
            nonce.secret().to_string()
        )
        .execute(&db)
        .await?;

        Ok(Redirect::to(auth_url.as_str()).into_response())
    }
    Router::new().route("/login", get(handler))
}
