//! Response struct for the Authorize OAuth API

use serde::Deserialize;

/// This is a model struct for AuthorizeOAuthResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct AuthorizeOAuthResponse {
    /// A valid authorization code. Authorization codes are exchanged for OAuth access tokens
    /// with the ObtainToken endpoint.
    /// Max Length 191
    pub code: Option<String>,
    /// The same value specified in the request.
    /// Min Length 1, Max Length 2048
    pub state: Option<String>,
}
