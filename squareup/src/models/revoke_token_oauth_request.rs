//! Request body struct for the Revoke Token OAuth API

use serde::Serialize;

/// This is a model struct for RevokeTokenOAuthRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct RevokeTokenOAuthRequest {
    /// The Square-issued ID for your application, which is available on the OAuth page in the
    /// Developer Dashboard.
    /// Max Length 191
    pub client_id: String,
    /// The access token of the merchant whose token you want to revoke. Do not provide a value for
    /// merchant_id if you provide this parameter.
    /// Min Length 2, Max Length 1024
    pub access_token: Option<String>,
    /// The ID of the merchant whose token you want to revoke. Do not provide a value for
    /// access_token if you provide this parameter.
    pub merchant_id: Option<String>,
    /// If true, terminate the given single access token, but do not terminate the entire
    /// authorization. Default: false
    pub revoke_only_access_token: Option<bool>,
}
