//! Model struct for AuthorizeOAuthParameters type

use crate::models::enums::{Locale, OAuthPermission};
use std::fmt::Display;

#[derive(Clone, Debug, Default)]
pub struct AuthorizeOAuthParameters {
    /// The Square-issued ID for your application, which is available on the OAuth page for your
    /// application in the Developer Dashboard.
    pub client_id: String,
    /// A space-separated list of the permissions that the application is requesting.
    /// Default: "MerchantProfileRead PaymentsRead SettlementsRead BankAccountsRead"
    pub scope: Option<Vec<OAuthPermission>>,
    /// The locale to present the permission request form in. Square detects the appropriate locale
    /// automatically. Only provide this value if the application can definitively determine the
    /// preferred locale.
    pub locale: Option<Locale>,
    /// If false, the user must log in to their Square account to view the Permission Request form,
    /// even if they already have a valid user session. This value has no effect in the
    /// Square Sandbox. Default: true
    pub session: Option<bool>,
    /// When provided, state is passed to the configured redirect URL after the Permission
    /// Request form is submitted. You can include state and verify its value to help protect
    /// against cross-site request forgery.
    pub state: Option<String>,
    /// When provided, the OAuth flow uses PKCE to authorize. The code_challenge will be associated
    /// with the AuthorizationCode and a code_verifier will need to passed in to obtain the access
    /// token.
    pub code_challenge: Option<String>,
    /// The redirect URL assigned on the OAuth page for your application in the Developer Dashboard.
    /// This field is required to use a dynamic port at runtime (PKCE only). To use a dynamic port,
    /// use the literal "" as a placeholder for a port in the Redirect URL box in the Developer
    /// Dashboard, for example, http://localhost:. When you call the Authorize endpoint from an
    /// application, pass in the actual port in this field. For example:
    /// https://connect.squareup.com/oauth2/authorize?client_id={YOUR_APP_ID}&scope=MERCHANT_PROFILE_READ&redirect_uri=http://localhost:8000
    pub redirect_uri: Option<String>,
}

impl AuthorizeOAuthParameters {
    pub fn to_query_string(&self) -> String {
        self.to_string()
    }
}

impl From<AuthorizeOAuthParameters> for String {
    fn from(authorize_oauth_parameters: AuthorizeOAuthParameters) -> Self {
        authorize_oauth_parameters.to_string()
    }
}

impl Display for AuthorizeOAuthParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut params = Vec::new();

        params.push(format!("client_id={}", self.client_id));

        if let Some(scope) = &self.scope {
            let mut scope_string_vec: Vec<String> = Vec::new();
            for permission in scope {
                scope_string_vec.push(serde_json::to_string(permission).unwrap());
            }
            params.push(format!("scope={}", scope_string_vec.join(" ")));
        }

        if let Some(locale) = &self.locale {
            params.push(format!("locale={:?}", locale));
        }

        if let Some(session) = &self.session {
            params.push(format!("session={}", session));
        }

        if let Some(state) = &self.state {
            params.push(format!("state={}", state));
        }

        if let Some(code_challenge) = &self.code_challenge {
            params.push(format!("code_challenge={}", code_challenge));
        }

        if let Some(redirect_uri) = &self.redirect_uri {
            params.push(format!("redirect_uri={}", redirect_uri));
        }

        let str = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        write!(f, "{}", str)
    }
}
