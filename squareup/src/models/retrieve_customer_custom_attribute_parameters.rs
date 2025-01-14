//! Model struct for RetrieveCustomerCustomAttributeParameters (query parameters)

use std::fmt::Display;

/// This is a model struct for RetrieveCustomerCustomAttributeParameters (query parameters)
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RetrieveCustomerCustomAttributeParameters {
    /// Indicates whether to return the custom attribute definition in the `definition` field
    /// of the custom attribute. Default: `false`.
    pub with_definition: Option<bool>,
    /// The current version of the custom attribute, used for strongly consistent reads.
    pub version: Option<i32>,
}

impl RetrieveCustomerCustomAttributeParameters {
    /// Converts the struct to a query string.
    pub fn to_query_string(&self) -> String {
        self.to_string()
    }
}

impl From<RetrieveCustomerCustomAttributeParameters> for String {
    fn from(params: RetrieveCustomerCustomAttributeParameters) -> Self {
        params.to_string()
    }
}

impl Display for RetrieveCustomerCustomAttributeParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut params = Vec::new();

        if let Some(with_definition) = &self.with_definition {
            params.push(format!("with_definition={}", with_definition));
        }

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
