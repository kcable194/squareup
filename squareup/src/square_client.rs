//! Gateway for the API

use crate::{config::Configuration, http::client::HttpClient, models::errors::ApiError};

/// Gateway struct for the library.
/// This struct acts as a factory for Apis.
/// It holds the state of the SDK.
#[derive(Clone)]
pub struct SquareClient {
    pub http_client: HttpClient,
    pub config: Configuration,
}

impl SquareClient {
    pub fn try_new(config: Configuration) -> Result<SquareClient, ApiError> {
        let http_client = HttpClient::try_new(&config.http_client_config)?;

        let client: SquareClient = Self {
            http_client,
            config,
        };

        Ok(client)
    }
}
