//! Representation of HTTP API response

use log::{error, warn};
use reqwest::Response;
use serde::de::DeserializeOwned;

use crate::models::errors::{ErrorResponse, SquareApiError};

/// Representation of HTTP API response.
///
/// Wraps Reqwest lib's Response
pub struct HttpResponse {
    inner: Response,
}

impl HttpResponse {
    pub fn new(inner: Response) -> Self {
        Self { inner }
    }

    pub fn is_success(&self) -> bool {
        self.inner.status().is_success()
    }

    pub async fn deserialize<T: DeserializeOwned>(self) -> Result<T, SquareApiError> {
        if self.is_success() {
            Ok(self.json().await?)
        } else {
            let err_response_res: Result<ErrorResponse, SquareApiError> = self.json().await;
            match err_response_res {
                Ok(error_response) => {
                    let api_error = SquareApiError::with_response_errors(
                        "Error response",
                        &error_response.errors,
                    );
                    warn!("{:?}", api_error);
                    Err(api_error)
                }
                Err(e) => {
                    let msg = format!("Error deserializing response errors: {}", e);
                    error!("{}", msg);
                    Err(SquareApiError::new(&msg))
                }
            }
        }
    }

    async fn json<T: DeserializeOwned>(self) -> Result<T, SquareApiError> {
        use serde_path_to_error;
        match self.inner.bytes().await {
            Err(e) => {
                let msg = format!("Error deserializing: {}", e);
                Err(SquareApiError::new(&msg))
            },
            Ok(full) => {
                let mut jd = serde_json::Deserializer::from_slice(&full);
                serde_path_to_error::deserialize(&mut jd).map_err(|e| {
                    let msg = format!("Error deserializing: {}", e);
                    SquareApiError::new(&msg)
                })
            }
        }
    }
}
