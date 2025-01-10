//! Model struct for RetrieveCustomerCustomAttributeDefinitionParameters (query parameters)

use std::fmt::Display;

/// This is a model struct for RetrieveCustomerCustomAttributeDefinitionParameters (query parameters)
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RetrieveCustomerCustomAttributeDefinitionParameters {
    /// The current version of the custom attribute definition.
    /// Used for strongly consistent reads to guarantee up-to-date data.
    pub version: Option<i32>,
}

impl RetrieveCustomerCustomAttributeDefinitionParameters {
    /// Converts the struct to a query string.
    pub fn to_query_string(&self) -> String {
        self.to_string()
    }
}

impl From<RetrieveCustomerCustomAttributeDefinitionParameters> for String {
    fn from(params: RetrieveCustomerCustomAttributeDefinitionParameters) -> Self {
        params.to_string()
    }
}

impl Display for RetrieveCustomerCustomAttributeDefinitionParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut params = Vec::new();

        if let Some(version) = &self.version {
            params.push(format!("version={}", version));
        }

        let str = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        write!(f, "{}", str)
    }
}
