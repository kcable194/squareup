//! Model struct for RevokeTokenOAuthResponse type

use super::errors::Error;
use serde::Deserialize;

/// This is a model struct for RevokeTokenOAuthResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct RevokeTokenOAuthResponse {
    /// If the request is successful, this is true.
    pub success: Option<bool>,
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
}
