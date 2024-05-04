use axum::response::IntoResponse;
use axum_extra::typed_header::TypedHeaderRejection;
use openidconnect::{
    core::CoreErrorResponseType, reqwest::AsyncHttpClientError, ClaimsVerificationError,
    ConfigurationError, RequestTokenError, SigningError, StandardErrorResponse,
    StandardTokenResponse, UserInfoError,
};
use thiserror::Error;
use tracing::error;

use crate::web::Web;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Request token error: {0}")]
    RequestToken(
        #[from]
        RequestTokenError<AsyncHttpClientError, StandardErrorResponse<CoreErrorResponseType>>,
    ),
    #[error("Missing token")]
    MissingToken,
    #[error("Claims verification error: {0}")]
    Claims(#[from] ClaimsVerificationError),
    #[error("Signing error: {0}")]
    Signing(#[from] SigningError),
    #[error("Access token mismatch")]
    AccessTokenMismatch,
    #[error("Configuration error: {0}")]
    Configuration(#[from] ConfigurationError),
    #[error("User info error: {0}")]
    UserInfo(#[from] UserInfoError<AsyncHttpClientError>),
    #[error("Typed header rejection")]
    TypedHeader(#[from] TypedHeaderRejection),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::Database(e) => {
                error!("{e}");
                Web::bad_request("Database error", "Provided query is invalid")
            }
            Error::RequestToken(e) => {
                error!("{e}");
                Web::internal_error(
                    "Internal error",
                    "Something wrong with the authentication mechanism",
                )
            }
            Error::MissingToken => {
                error!("Missing token error");
                Web::internal_error(
                    "Missing token",
                    "The id token cannot be retrieved, please check the IdP settings",
                )
            }
            Error::Claims(e) => {
                error!("{e}");
                Web::internal_error("Claims verification error", "The claims cannot be verified")
            }
            Error::Signing(e) => {
                error!("{e}");
                Web::internal_error("Signing error", "Cannot sign")
            }
            Error::AccessTokenMismatch => {
                error!("Signed access token and access token hash does not match");
                Web::internal_error("Access token mismatch", "The access tokens are incorrect")
            }
            Error::Configuration(e) => {
                error!("{e}");
                Web::internal_error("Configuration error", "Your IdP configuration has issues")
            }
            Error::UserInfo(e) => {
                error!("{e}");
                Web::bad_request("User info error", "Cannot get user info from the IdP")
            }
            Error::TypedHeader(e) => {
                error!("{e}");
                Web::bad_request("Header rejected", "The header is invalid")
            }
        }
    }
}
