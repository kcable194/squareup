//! Request body struct for the Obtain Token OAuth API

use crate::models::enums::{GrantType, OAuthPermission};
use serde::Serialize;

/// This is a model struct for ObtainTokenOAuthRequest type.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ObtainTokenOAuthRequest {
    /// The Square-issued ID for your application, which is available on the OAuth page in the
    /// Developer Dashboard.
    /// Max Length 191
    pub client_id: String,
    /// The Square-issued application secret for your application, which is available on the OAuth
    /// page in the Developer Dashboard. This parameter is only required when you're not using the
    /// OAuth PKCE (Proof Key for Code Exchange) flow. The PKCE flow requires a code_verifier
    /// instead of a client_secret when grant_type is set to AuthorizationCode. If grant_type is
    /// set to RefreshToken and the RefreshToken is obtained using PKCE, the PKCE flow only
    /// requires client_id,  grant_type, and RefreshToken.
    /// Min Length 2, Max Length 1024
    pub client_secret: Option<String>,
    /// The authorization code to exchange. This code is required if grant_type is set to
    /// AuthorizationCode to indicate that the application wants to exchange an authorization
    /// code for an OAuth access token.
    /// Max Length 191
    pub code: Option<String>,
    /// The redirect URL assigned on the OAuth page for your application in the Developer Dashboard.
    /// Max Length 2048
    pub redirect_uri: Option<String>,
    /// Specifies the method to request an OAuth access token. Valid values are AuthorizationCode,
    /// RefreshToken, and MigrationToken.
    /// Min Length 10, Max Length 20
    pub grant_type: GrantType,
    /// A valid refresh token for generating a new OAuth access token.
    ///
    /// A valid refresh token is required if grant_type is set to RefreshToken to indicate that
    /// the application wants a replacement for an expired OAuth access token.
    /// Min Length 2, Max Length 1024
    pub refresh_token: Option<String>,
    /// A legacy OAuth access token obtained using a Connect API version prior to 2019-03-13.
    /// This parameter is required if grant_type is set to MigrationToken to indicate that the
    /// application wants to get a replacement OAuth access token. The response also returns a
    /// refresh token. For more information, see Migrate to Using Refresh Tokens.
    /// Min Length 2, Max Length 1024
    #[deprecated]
    pub migration_token: Option<String>,
    /// A JSON list of strings representing the permissions that the application is requesting.
    /// For example, "["MERCHANT_PROFILE_READ","PAYMENTS_READ","BANK_ACCOUNTS_READ"]".
    ///
    /// The access token returned in the response is granted the permissions that comprise the
    /// intersection between the requested list of permissions and those that belong to the
    /// provided refresh token.
    pub scopes: Option<Vec<OAuthPermission>>,
    /// A Boolean indicating a request for a short-lived access token.
    ///
    /// The short-lived access token returned in the response expires in 24 hours.
    pub short_lived: Option<bool>,
    /// Must be provided when using the PKCE OAuth flow if grant_type is set to AuthorizationCode.
    /// The code_verifier is used to verify against the code_challenge associated with the
    /// AuthorizationCode.
    pub code_verifier: Option<String>,
}
