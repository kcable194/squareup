//! The OAuth API lets you connect your application to a seller's account using OAuth.
//!
//! The Square OAuth API uses the OAuth 2 protocol to get permission from the owner of the seller
//! account to manage specific types of resources in that account. This is the process where client
//! applications obtain an authorization code that is then redeemed to get an access token and
//! refresh token. These tokens allow you to manage resources for a seller and are used when
//! calling the Square APIs. The OAuth API lets you request specific permissions from Square
//! sellers to manage their resources and get access tokens to call the Square APIs on their
//! behalf. Usually, you make OAuth part of your setup process when onboarding a Square seller
//! to your application.

use crate::models::{
    AuthorizeOAuthParameters, AuthorizeOAuthResponse, ObtainTokenOAuthRequest,
    ObtainTokenOAuthResponse, RetrieveTokenStatusResponse, RevokeTokenOAuthRequest,
    RevokeTokenOAuthResponse,
};
use crate::{
    config::Configuration, http::client::HttpClient, models::errors::SquareApiError, SquareClient,
};
use reqwest::header::{HeaderValue, AUTHORIZATION};

const DEFAULT_URI: &str = "/oauth2";

pub struct OAuthApi {
    /// App config information
    config: Configuration,
    /// HTTP Client for requests to the OAuth API endpoints
    http_client: HttpClient,
}

impl OAuthApi {
    /// Instantiates a new `OAuthApi`
    pub fn new(square_client: SquareClient) -> OAuthApi {
        OAuthApi {
            config: square_client.config,
            http_client: square_client.http_client,
        }
    }

    /// As part of a URL sent to a seller to authorize permissions for the developer, Authorize
    /// displays an authorization page and a list of requested permissions.
    ///
    /// The completed URL looks similar to the following example: https://connect.squareup.com/oauth2/authorize?client_id={YOUR_APP_ID}&scope=CustomersWrite+CustomersRead&session=False&state=82201dd8d83d23cc8a48caf52b
    ///
    /// Learn more (https://developer.squareup.com/reference/square/o-auth-api/authorize).
    pub async fn authorize(
        &self,
        params: &AuthorizeOAuthParameters,
    ) -> Result<AuthorizeOAuthResponse, SquareApiError> {
        let url = format!("{}/authorize{}", &self.url(), params.to_query_string());
        let response = self.http_client.get(&url).await?;

        response.deserialize().await
    }

    /// Revokes an access token generated with the OAuth flow.
    ///
    /// If an account has more than one OAuth access token for your application, this endpoint
    /// revokes all of them, regardless of which token you specify.
    ///
    /// Important: The Authorization header for this endpoint must have the following format:
    /// Authorization: Client APPLICATION_SECRET
    pub async fn revoke_token(
        &self,
        application_secret: &str,
        body: &RevokeTokenOAuthRequest,
    ) -> Result<RevokeTokenOAuthResponse, SquareApiError> {
        let url = format!("{}/revoke", self.url());
        let header_value = HeaderValue::try_from(format!("Client {}", application_secret))
            .map_err(|e| SquareApiError::new(e.to_string().as_str()))?; // TODO ensure this actually overrides the auth
        let response = self
            .http_client
            .post_with_header(&url, body, AUTHORIZATION, header_value)
            .await?;

        response.deserialize().await
    }

    /// Returns an OAuth access token and a refresh token unless the short_lived parameter is set
    /// to true, in which case the endpoint returns only an access token.
    ///
    /// The grant_type parameter specifies the type of OAuth request. If grant_type is
    /// AuthorizationCode, you must include the authorization code you received when a seller
    /// granted you authorization. If grant_type is RefreshToken, you must provide a valid
    /// refresh token. If you're using an old version of the Square APIs (prior to March 13, 2019),
    /// grant_type can be MigrationToken and you must provide a valid migration token.
    ///
    /// You can use the scopes parameter to limit the set of permissions granted to the access
    /// token and refresh token. You can use the short_lived parameter to create an access token
    /// that expires in 24 hours.
    ///
    /// Note: OAuth tokens should be encrypted and stored on a secure server. Application clients
    /// should never interact directly with OAuth tokens.
    pub async fn obtain_token(
        &self,
        body: &ObtainTokenOAuthRequest,
    ) -> Result<ObtainTokenOAuthResponse, SquareApiError> {
        let url = format!("{}/token", self.url());
        let response = self.http_client.post(&url, body).await?;

        response.deserialize().await
    }

    /// Returns information about an OAuth access token or an application’s personal access token.
    /// Add the access token to the Authorization header of the request.
    ///
    /// Important: The Authorization header you provide to this endpoint must have the following
    /// format: Authorization: Bearer ACCESS_TOKEN
    /// where ACCESS_TOKEN is a valid production authorization credential. If the access token is
    /// expired or not a valid access token, the endpoint returns an UNAUTHORIZED error.
    pub async fn retrieve_token_status(
        &self,
        access_token: impl AsRef<str>,
    ) -> Result<RetrieveTokenStatusResponse, SquareApiError> {
        let url = format!("{}/token", self.url());
        let header_value = HeaderValue::try_from(format!("Bearer {}", access_token.as_ref()))
            .map_err(|e| SquareApiError::new(e.to_string().as_str()))?;
        let response = self
            .http_client
            .post_with_header(&url, "", AUTHORIZATION, header_value)
            .await?;

        response.deserialize().await
    }

    /// Constructs the basic entity URL including domain and entity path. Any additional path
    /// elements (e.g. path parameters) will need to be appended to this URL.
    fn url(&self) -> String {
        format!("{}{}", &self.config.get_base_url(), DEFAULT_URI)
    }
}
