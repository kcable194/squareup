//! Query String parameters for the Retrieve Subscription API

use std::fmt::Display;

/// This is a model struct for RetrieveSubscriptionParameters (query parameters)
#[derive(Clone, Debug, Default)]
pub struct RetrieveSubscriptionParameters {
    /// A query parameter to specify related information to be included in the response.
    ///
    /// The supported query parameter values are:
    ///
    /// * actions: to include scheduled actions on the targeted subscription.
    pub include: Option<String>,
}

impl RetrieveSubscriptionParameters {
    pub fn to_query_string(&self) -> String {
        self.to_string()
    }
}

impl From<RetrieveSubscriptionParameters> for String {
    fn from(retrieve_subscription_parameters: RetrieveSubscriptionParameters) -> Self {
        retrieve_subscription_parameters.to_string()
    }
}

impl Display for RetrieveSubscriptionParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut params = Vec::new();

        if let Some(include) = &self.include {
            params.push(format!("include={}", include));
        }

        let str = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        write!(f, "{}", str)
    }
}
