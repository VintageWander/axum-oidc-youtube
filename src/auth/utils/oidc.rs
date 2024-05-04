use std::time::Duration;

use openidconnect::{
    core::{CoreClient, CoreProviderMetadata},
    reqwest::async_http_client,
};
use tokio::time::sleep;
use tracing::error;

use crate::env::{client_id, issuer_url, redirect_url};

pub async fn new_oidc_client() -> CoreClient {
    let provider_metadata = loop {
        match CoreProviderMetadata::discover_async(issuer_url(), async_http_client).await {
            Ok(metadata) => break metadata,
            Err(error) => {
                error!("Discovery error: {error:#?}");
                sleep(Duration::from_secs(10)).await
            }
        }
    };
    CoreClient::from_provider_metadata(provider_metadata, client_id(), None)
        .set_redirect_uri(redirect_url())
}
