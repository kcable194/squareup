//! Model struct for ObtainTokenOAuthResponse type

use super::errors::Error;
use crate::models::Iso8601Date;
use serde::Deserialize;

/// This is a model struct for ObtainTokenOAuthResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct ObtainTokenOAuthResponse {
    /// A valid OAuth access token. Provide the access token in a header with every request to
    /// Connect API endpoints. For more information, see OAuth API: Walkthrough.
    /// Min Length 2, Max Length 1024
    pub access_token: Option<String>,
    /// This value is always bearer.
    /// Min Length 2, Max Length 10
    pub token_type: Option<String>,
    /// The date when the access_token expires, in ISO 8601 format.
    /// Min Length 20, Max Length 48
    pub expires_at: Option<Iso8601Date>,
    /// The ID of the authorizing merchant's business.
    /// Min Length 8, Max Length 191
    pub merchant_id: Option<String>,
    /// LEGACY FIELD. The ID of a subscription plan the merchant signed up for. The ID is only
    /// present if the merchant signed up for a subscription plan during authorization.
    #[deprecated]
    pub subscription_id: Option<String>,
    /// LEGACY FIELD. The ID of the subscription plan the merchant signed up for. The ID is only
    /// present if the merchant signed up for a subscription plan during authorization.
    #[deprecated]
    pub plan_id: Option<String>,
    /// The OpenID token belonging to this person. This token is only present if the OPENID scope
    /// is included in the authorization request.
    #[deprecated]
    pub id_token: Option<String>,
    /// A refresh token. For more information, see Refresh, Revoke, and Limit the Scope of
    /// OAuth Tokens.
    /// Min Length 2, Max Length 1024
    pub refresh_token: Option<String>,
    /// A Boolean indicating that the access token is a short-lived access token. The short-lived
    /// access token returned in the response expires in 24 hours.
    pub short_lived: Option<bool>,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
    /// The date when the refresh_token expires, in ISO 8601 format.
    /// Min Length 20, Max Length 48
    pub refresh_token_expires_at: Option<Iso8601Date>,
}
